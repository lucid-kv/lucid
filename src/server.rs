use std::io::Read;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::RwLock;

use hyper::header::*;
use jsonwebtoken::{decode, Validation};
use nickel::{*, HttpRouter, Middleware, MiddlewareResult, Nickel, Options, Request, Response, StaticFilesHandler};
use nickel::hyper::method::Method;
use nickel::status::StatusCode;

use crate::configuration::Configuration;
use crate::kvstore::KvStore;
use crate::logger::{Logger, LogLevel};

// TODO: passing configuration to Server
pub struct Server {
    endpoint: String,
    use_tls: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    code: i32,
    message: &'static str,
    details: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
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
}

impl<D> Middleware<D> for KvStoreMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, mut res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        // Get the request body and retrieve the KV store
        let store = &*self.store.write().unwrap();
        let mut buffer = Vec::new();
        let body_size = req.origin.read_to_end(&mut buffer).unwrap();

        // Define some response headers
        // TODO: use crate version
        res.set(Server("Lucid 0.1.2".to_string()));

        match req.origin.headers.get::<Authorization<Bearer>>() {
            // TODO: get secret from configuration file
            Some(header) => match decode::<Claims>(&header.token, "0cccdfddd23f9f740c9620a094daf1b260436059924cec32622f0e7ebc99bbe5".as_ref(), &Validation::default()) {
                Ok(_bearer) => match self.http_verb {
                    Method::Head => match req.param("key") {
                        Some(key) => match &store.get(key.to_string()) {
                            Some(_) => {
                                res.set(StatusCode::Ok);
                                res.send("")
                            },
                            None => {
                                res.set(StatusCode::NotFound);
                                res.send("")
                            }
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { code: 102, message: "Missing key parameter.", details: None }).unwrap())
                        }
                    },
                    Method::Put => {
                        if body_size == 0 {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            return res.send(serde_json::to_string_pretty(&ErrorMessage { code: 101, message: "Missing request body.", details: None }).unwrap());
                        }
                        match req.param("key") {
                            Some(key) => if buffer.len() < 7340032 {
                                store.set(key.to_string(), buffer);
                                res.set(StatusCode::Ok);
                                res.send("")
                            } else {
                                res.set(StatusCode::BadRequest).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { code: 103, message: "The maximum allowed value size is 7 Mb.", details: None }).unwrap())
                            },
                            None => {
                                res.set(StatusCode::BadRequest).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { code: 102, message: "Missing key parameter.", details: None }).unwrap())
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
                                    // TODO: use create version
                                    return res.send(serde_json::to_string_pretty(&ErrorMessage { code: 0, message: "Lucid Version 0.1.2", details: None }).unwrap());
                                }
                                res.set(StatusCode::NotFound).set(MediaType::Json);
                                res.send(serde_json::to_string_pretty(&ErrorMessage { code: 104, message: "The specified key does not exists.", details: None }).unwrap())
                            }
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { code: 102, message: "Missing key parameter.", details: None }).unwrap())
                        }
                    },
                    Method::Delete => match req.param("key") {
                        Some(key) => {
                            store.drop(key.to_string());
                            res.set(StatusCode::Ok);
                            res.send("")
                        },
                        None => {
                            res.set(StatusCode::BadRequest).set(MediaType::Json);
                            res.send(serde_json::to_string_pretty(&ErrorMessage { code: 102, message: "Missing key parameter.", details: None }).unwrap())
                        }
                    },
                    _ => {
                        res.set(StatusCode::MethodNotAllowed).set(MediaType::Json);
                        res.send(serde_json::to_string_pretty(&ErrorMessage { code: 105, message: "Method not allowed, maybe in the future :)", details: None }).unwrap())
                    }
                },
                Err(e) => {
                    res.set(StatusCode::InternalServerError).set(MediaType::Json);
                    return res.send(serde_json::to_string_pretty(&ErrorMessage { code: 100, message: "Unable to decrypt JWT token.", details: Some(e.to_string()) }).unwrap());
                }
            },
            None => {
                res.set(StatusCode::Unauthorized).set(MediaType::Json);
                return res.send(serde_json::to_string_pretty(&ErrorMessage { code: 100, message: "Missing JWT token.", details: None }).unwrap());
            }
        }
    }
}

impl Server
{
    pub fn new() -> Server
    {
        Server {
            endpoint: format!("{}:7021", Ipv4Addr::LOCALHOST),
            use_tls: false,
        }
    }

    pub fn configure(&mut self, configuration: &Configuration) {
        self.endpoint = configuration.endpoint.to_owned().replace("\"", "");
        self.use_tls = configuration.use_tls;
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

        let store = Arc::new(RwLock::new(KvStore::default()));

        server.utilize(middleware_logging);

        // CORS
        server.utilize(middleware_cors);
        server.options("**", middleware!(""));

        // Web UI
        server.utilize(self.router_webui());
        server.utilize(StaticFilesHandler::new("assets/"));
        server.utilize(StaticFilesHandler::new("webui/dist"));

        // Robots.txt
        server.get("/robots.txt", middleware!("User-agent: *\nDisallow: /"));

        // API Endpoints
        // TODO: change to server.head() (https://github.com/nickel-org/nickel.rs/issues/444)
        server.add_route(Method::Head, "/api/kv/:key", KvStoreMiddleware { http_verb: Method::Head, store: store.clone() });
        server.put("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Put, store: store.clone() });
        server.get("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Get, store: store.clone() });
        server.patch("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Patch, store: store.clone() });
        server.delete("/api/kv/:key", KvStoreMiddleware { http_verb: Method::Delete, store: store.clone() });

        // SSE Endpoints
        server.utilize(self.router_sse());

        // TODO: Implement HTTPS (https://github.com/nickel-org/nickel.rs/blob/master/examples/https.rs)

        match self.use_tls {
            true => {
//                use hyper::Server;
//                use hyper_openssl::OpensslServer;
//                let ssl = Openssl::with_cert_and_key("examples/assets/self_signed.crt", "examples/assets/key.pem").unwrap();
//                server.listen_https("127.0.0.1:7021", ssl);
            },
            false => match server.listen(&self.endpoint) {
                Ok(instance) => {
                    // TODO: move logging for using in https to
                    // TODO: try using server.log and getting owner
                    &self.log(LogLevel::Information, format!("Running Lucid server on {endpoint} | PID: {pid}", endpoint = instance.socket(), pid = std::process::id()).as_str(), None);
                    &self.log(LogLevel::Information, format!("Lucid API Endpoint: {scheme}://{endpoint}/api/", scheme = match self.use_tls {
                        true => "https",
                        false => "http"
                    }, endpoint = instance.socket()).as_str(), None);
                    &self.log(LogLevel::Information, "Use Ctrl+C to stop the server.", None);
                }
                Err(err) => {
                    &self.log(LogLevel::Error, "Unable to run Lucid server", Some(Box::leak(err).description()));
                }
            },
        }
    }
}