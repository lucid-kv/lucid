use std::collections::HashMap;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::RwLock;

use nickel::{*, Action, Continue, FormBody, Halt, HttpRouter, JsonBody, Middleware, MiddlewareResult, Nickel, NickelError, Options, Request, Response, StaticFilesHandler};
use nickel::hyper::method::Method;
use nickel::status::StatusCode;

use crate::configuration::Configuration;
use crate::kvstore::KvStore;
use crate::logger::{Logger, LogLevel};

pub struct Server {
    endpoint: String,
    use_tls: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    message: String,
    details: Option<String>
}

// TODO: move into implementation
fn handler_vuejs<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "Alex");
    res.render("webui/dist/index.tpl", &data)
}

fn handler_logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
//    let r = request.origin.;
//    println!("{}", 5);
    crate::logger::print(&LogLevel::Information, format!("{} {}", request.origin.method, request.origin.uri).as_ref());
    response.next_middleware()
}

struct KvStoreMiddleware {
    method: hyper::method::Method,
    store: Arc<RwLock<KvStore>>,
}

impl<D> Middleware<D> for KvStoreMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, mut res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        // TODO: validate JWT token
        let store = &*self.store.write().unwrap();
        let mut buffer = String::new();
        let body_size = req.origin.read_to_string(&mut buffer).unwrap();
        match self.method {
            Method::Head => match req.param("key") {
                Some(key) => match &store.get(key.to_string()) {
                    Some(_) => {
                        res.set(StatusCode::Ok);
                        res.send("")
                    },
                    None => {
                        res.set(StatusCode::NotFound).set(MediaType::Json);
                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key does not exists.".to_string(), details: None }).unwrap())
                    }
                },
                _ => {
                    res.set(StatusCode::BadRequest).set(MediaType::Json);
                    res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string(), details: None }).unwrap())
                }
            },
            Method::Put => {
                if body_size == 0 {
                    res.set(StatusCode::BadRequest).set(MediaType::Json);
                    return res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing request body.".to_string(), details: None }).unwrap());
                }
                match req.param("key") {
                    Some(key) => {
                        store.set(key.to_string(), buffer);
                        res.set(StatusCode::Ok);
                        res.send("")
                    },
                    _ => {
                        res.set(StatusCode::BadRequest).set(MediaType::Json);
                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string(), details: None }).unwrap())
                    }
                }
            },
            Method::Get => match req.param("key") {
                Some(key) => match store.get(key.to_string()) {
                    Some(value) => {
                        res.set(StatusCode::Ok);
                        res.send(value)
                    },
                    None => {
                        res.set(StatusCode::NotFound).set(MediaType::Json);
                        res.send(serde_json::to_string_pretty(&ErrorMessage { message: "The specified key does not exists.".to_string(), details: None }).unwrap())
                    }
                },
                _ => {
                    res.set(StatusCode::BadRequest).set(MediaType::Json);
                    res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string(), details: None }).unwrap())
                }
            },
            Method::Delete => match req.param("key") {
                Some(key) => {
                    store.drop(key.to_string());
                    res.set(StatusCode::Ok);
                    res.send("")
                },
                _ => {
                    res.set(StatusCode::BadRequest).set(MediaType::Json);
                    res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Missing key parameter.".to_string(), details: None }).unwrap())
                }
            },
            _ => {
                res.set(StatusCode::MethodNotAllowed).set(MediaType::Json);
                res.send(serde_json::to_string_pretty(&ErrorMessage { message: "Method not allowed, maybe in the future :)".to_string(), details: None }).unwrap())
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
        router.get("/", handler_vuejs);
        router
    }

    pub fn run(&self) {
        let mut server = Nickel::with_options(Options::default().output_on_listen(false));

        let store = Arc::new(RwLock::new(KvStore::default()));

        server.utilize(handler_logger);

        server.utilize(StaticFilesHandler::new("assets/"));
        server.utilize(StaticFilesHandler::new("webui/dist"));

        server.utilize(self.router_webui());

        // API Endpoints
        server.add_route(Method::Head, "/api/kv/:key", KvStoreMiddleware { method: Method::Head, store: store.clone() });
        server.put("/api/kv/:key", KvStoreMiddleware { method: Method::Put, store: store.clone() });
        server.get("/api/kv/:key", KvStoreMiddleware { method: Method::Get, store: store.clone() });
        server.patch("/api/kv/:key", KvStoreMiddleware { method: Method::Patch, store: store.clone() });
        server.delete("/api/kv/:key", KvStoreMiddleware { method: Method::Delete, store: store.clone() });

        // TODO: Implement HTTPS (https://github.com/nickel-org/nickel.rs/blob/master/examples/https.rs)

        match server.listen(&self.endpoint) {
            Ok(instance) => {
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
        }

//        if self.use_tls {
//            server.listen_https()
//        }
    }

    pub fn dispose(&self) {
        self.log(LogLevel::Information, "Stopping the Lucid server", None);
    }
}