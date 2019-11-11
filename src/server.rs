use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

use byte_unit::{Byte, ByteUnit};
use hyper::header::*;
use jsonwebtoken::{decode, Validation};
use nickel::{*, HttpRouter, Middleware, MiddlewareResult, Nickel, Options, Request, Response, StaticFilesHandler};
use nickel::hyper::method::Method;
use nickel::status::StatusCode;

use crate::configuration::*;
use crate::kvstore::KvStore;
use crate::logger::{Logger, LogLevel};

pub struct Server {
    configuration: Configuration
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
struct PatchValue {
    operation: String
}

fn middleware_webui<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    res.set(MediaType::Html);
    res.send_file("webui/dist/index.html")
}

fn middleware_logging<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    crate::logger::print(&LogLevel::Information, format!("{} {}", request.origin.method, request.origin.uri).as_ref());
    response.next_middleware()
}

fn middleware_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Methods", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"*".to_vec()]); //Origin, Authorization, X-Requested-With, Content-Type, Accept
    res.next_middleware()
}

struct KvStoreMiddleware {
    http_verb: hyper::method::Method,
    store: Arc<RwLock<KvStore>>,
    configuration: Configuration
}

impl<D> Middleware<D> for KvStoreMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, mut res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {

        // Get the request body and retrieve the KV store
        let store = &*self.store.write().unwrap();
        let mut buffer = Vec::new();
        let body_size = req.origin.read_to_end(&mut buffer).unwrap();

        // Set the server response header
        res.set(Server(format!("Lucid {}", crate_version!())));

        // TODO: handle authentication disabled
        match req.origin.headers.get::<Authorization<Bearer>>() {
            Some(header) => match decode::<Claims>(&header.token, self.configuration.authentication.secret_key.as_ref(), &Validation::default()) {
                Ok(_bearer) => match self.http_verb {
                    Method::Head => match req.param("key") {
                        Some(key) => match &store.get(key.to_string()) {
                            Some(_) => {
                                res.set(StatusCode::Ok);
                                res.send("")
                            },
                            None => {
                                res.set(StatusCode::NotFound).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key does not exists.".to_string() }).unwrap())
                            }
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string() }).unwrap())
                        }
                    },
                    Method::Put => {
                        if body_size == 0 {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            return res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing request body.".to_string() }).unwrap());
                        }

                        match req.param("key") {
                            Some(key) => if buffer.len() < self.configuration.store.max_limit as usize {
                                match store.set(key.to_string(), buffer) {
                                    None => {
                                        res.set(StatusCode::Created).set(MediaType::Json);
                                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key was successfully created.".to_string() }).unwrap())
                                    },
                                    Some(_) => {
                                        res.set(StatusCode::Ok).set(MediaType::Json);
                                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key was successfully updated.".to_string() }).unwrap())
                                    }
                                }
                            } else {
                                res.set(StatusCode::BadRequest).set(MediaType::Json);
                                let max_limit = Byte::from_bytes(self.configuration.store.max_limit as u128);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { message: format!("The maximum allowed value size is {}.", max_limit.get_appropriate_unit(true)) }).unwrap())
                            },
                            None => {
                                res.set(StatusCode::BadRequest).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string() }).unwrap())
                            }
                        }
                    },
                    Method::Get => match req.param("key") {
                        // TODO: check query string, for getting metadata

                        Some(key) => match store.get(key.to_string()) {
                            Some(value) => {
                                res.set(StatusCode::Ok).set(MediaType::Txt);
                                res.send(value)
                            },
                            None => {
                                // TODO: found a better name / location
                                if req.param("key").unwrap() == "check-token" {
                                    res.set(StatusCode::Ok).set(MediaType::Json);
                                    return res.send(serde_json::to_string_pretty(&ErrorMessage { message: format!("Lucid {}", crate_version!()) }).unwrap());
                                }
                                res.set(StatusCode::NotFound).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key does not exists.".to_string() }).unwrap())
                            }
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string() }).unwrap())
                        }
                    },
                    Method::Delete => match req.param("key") {
                        Some(key) => {
                            store.drop(key.to_string());
                            res.set(StatusCode::Ok);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key and it's data was successfully deleted.".to_string() }).unwrap())
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string() }).unwrap())
                        }
                    },
                    Method::Patch => match req.param("key") {
                        Some(key) => {
                            // TODO: Operations to implement: LOCK, UNLOCK, INCREMENT, DECREMENT, EXPIRE

                            if let Ok(json_body) = std::str::from_utf8((*buffer).as_ref()) {
                                match serde_json::from_str::<PatchValue>(json_body) {
                                    Ok(patch_value) => {
                                        match patch_value.operation.as_str() {
                                            "lock" | "unlock" => {
                                                let r = store.switch_lock(key.to_string(), true);
                                                println!("{}", r);
                                            }
                                            _ => ()
                                        }
                                    },
                                    Err(e) => {
                                        println!("{}", e);
                                    }
                                }
                            }
                            res.set(StatusCode::Ok);
                            res.send("")
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string() }).unwrap())
                        }
                    },
                    _ => {
                        res.set(StatusCode::MethodNotAllowed).set(MediaType::Json);
                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Method not allowed, maybe in the future :)".to_string() }).unwrap())
                    }
                },
                Err(_) => {
                    res.set(StatusCode::InternalServerError).set(MediaType::Json);
                    return res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Unable to decrypt JWT token.".to_string() }).unwrap());  //, details: Some(e.to_string())
                }
            },
            None => {
                res.set(StatusCode::Unauthorized).set(MediaType::Json);
                return res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing JWT token.".to_string() }).unwrap());
            }
        }
    }
}

impl Server
{
    pub fn new() -> Server
    {
        Server {
            configuration: Configuration::default()
        }
    }

    pub fn configure(&mut self, configuration: Configuration) {
        self.configuration = configuration;
    }

    fn router_webui(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.get("/", middleware_webui);
        router.get("/api/ui/version", middleware!(format!("Lucid Version {}", crate_version!())));
        router
    }

    fn router_sse(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.get("/sse/test", middleware! { |_request, mut response|
            response.set(StatusCode::BadRequest).set(MediaType::Json);
            "lol"
        });
        router
    }

    pub fn run(&self) {
        let server_options = Options::default()
            .thread_count(None) // TODO: [Optimisation] improve this
            .output_on_listen(false);

        let mut server = Nickel::with_options(server_options);

        let store = Arc::new(RwLock::new(KvStore::new()));

        server.utilize(middleware_logging);

        // CORS
        server.utilize(middleware_cors);
        server.options("**", middleware!(""));

        // Web UI
        if self.configuration.webui.enabled {
            server.utilize(self.router_webui());
            server.utilize(StaticFilesHandler::new("assets/"));
            server.utilize(StaticFilesHandler::new("webui/dist"));
        }

        // TODO: maybe define if set in the configuration file
        // Robots.txt
        server.get("/robots.txt", middleware!("User-agent: *\nDisallow: /"));

        // API Endpoints
        // TODO: change to server.head() (https://github.com/nickel-org/nickel.rs/issues/444)
        server.add_route(Method::Head, "/api/kv/:key", KvStoreMiddleware { http_verb: Method::Head, store: store.clone(), configuration: self.configuration.clone() });
        server.put("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Put, store: store.clone(), configuration: self.configuration.clone() });
        server.get("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Get, store: store.clone(), configuration: self.configuration.clone() });
        server.patch("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Patch, store: store.clone(), configuration: self.configuration.clone() });
        server.delete("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Delete, store: store.clone(), configuration: self.configuration.clone() });

        // SSE Endpoints
        server.utilize(self.router_sse());

        // TODO: Implement HTTPS (https://github.com/nickel-org/nickel.rs/blob/master/examples/https.rs)
        match self.configuration.default.use_ssl {
            true => {
//                use hyper::Server;
//                use hyper_openssl::OpensslServer;
//                let ssl = Openssl::with_cert_and_key("examples/assets/self_signed.crt", "examples/assets/key.pem").unwrap();
//                server.listen_https("127.0.0.1:7021", ssl);
            },
            false => match server.listen(self.configuration.clone().get_bind_endpoint()) {
                Ok(instance) => {
                    // TODO: move logging for using in https to
                    // TODO: try using server.log and getting owner
                    &self.log(LogLevel::Information, format!("Running Lucid server on {endpoint} | PID: {pid}", endpoint = instance.socket(), pid = std::process::id()).as_str(), None);
                    &self.log(LogLevel::Information, format!("Lucid API Endpoint: http://{endpoint}/api/", endpoint = instance.socket()).as_str(), None);
                    if self.configuration.webui.enabled {
                        &self.log(LogLevel::Information, format!("Lucid Web UI Path: http://{endpoint}/", endpoint = instance.socket()).as_str(), None);
                    }
                    &self.log(LogLevel::Information, "Use Ctrl+C to stop the server.", None);
                }
                Err(err) => {
                    &self.log(LogLevel::Error, "Unable to run Lucid server", Some(Box::leak(err).description()));
                }
            },
        }
    }
}