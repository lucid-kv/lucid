use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::Ipv4Addr;
use std::str::FromStr;

use nickel::{*, Action, Continue, Halt, HttpRouter, Middleware, MiddlewareResult, Nickel, NickelError, Options, Request, Response, StaticFilesHandler, JsonBody};
use nickel::status::StatusCode;
use nickel::status::StatusCode::NotFound;
use yaml_rust::YamlLoader;

use crate::configuration::Configuration;
use crate::logger::{Logger, LogLevel};
use nickel::hyper::method::Method;

use crate::kvstore::KvStore;

pub struct Server {
    endpoint: String,
    use_tls: bool,
    store: KvStore
}

#[derive(Debug, Serialize, Deserialize)]
struct EntitySet {
    value: String,
    expiration:  i32,
}

// TODO: move into implementation
fn handler_vuejs<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "Alex");
    res.render("webui/dist/index.tpl", &data)
}

fn handler_logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    crate::logger::print(&LogLevel::Information, format!("{} {}", request.origin.method, request.origin.uri).as_ref());
    response.next_middleware()
}

fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello World")
}

impl Server
{
    pub fn default() -> Server
    {
        Server {
            endpoint: format!("{}:7021", Ipv4Addr::LOCALHOST),
            use_tls: false,
            store: KvStore::default()
        }
    }

    pub fn configure(&mut self, configuration: &Configuration) {
        self.endpoint = configuration.endpoint.to_owned().replace("\"", "");
        self.use_tls = configuration.use_tls;
    }

    fn router_webui(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.get("/**", handler_vuejs);
        router
    }

    fn hello_world<'mw>(&self, _req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
        match _req.param("key") {
            Some(key) => {
    //                self.store.set(String::from("test"), String::from("test"));
            },
            _ => {
            }  // TODO: return error, missing key parameter
        }
        res.send("Hello World")
    }

    fn router_api(&self) -> nickel::Router {
        let mut router = Nickel::router();
        
        // let m = |req, res| self.hello_world(req, res);

        // router.add_route(Method::Head, "/api/kv/:key", m);

        // SET/GET/EXIST
        // LOCK/UNLOCK
        // EXPIRE/UNEXPIRE
        // INCREMENT/DECREMENT

        // GET /api/kv/:key
        // POST /api/kv/:key
        // PUT /api/kv/:key
        // DELETE /api/kv/:key

        // SET (without key)
        // router.add_route(Method::Put, "/api/kv", middleware! {|request, response|
        //     "success"
        // });

        // SET (with key)
        // router.add_route(Method::Put, "/api/kv/:key", middleware! {|request, response|
        //    match request.param("key") {
        //        Some(key) => {
        //            self.store.set(String::from("test"), String::from("test"));
        //            "success"
        //        },
        //        _ => {
        //            "error"
        //        }  // TODO: return error, missing key parameter
        //    }
        // });
        
         /*middleware! {|request, response|
            println!("{:#?}", request.param("key"));
            "success"
        });*/

//        router.head("/api/kv/:key", middleware! {|request, response|
//            "success" 
//        });

        // GET /api/kv/:key
        // router.get("/api/kv/:key",  middleware!{ |request, response|
        //     "success"
//            let entity_set = request.json_as::<EntitySet>().unwrap();
//            match request.param("key") {
//                Some(key) => {
//                    format!("{:?}", entity_set.value)
//                },
//                _ => format!("success")
//            }
        // });

        // PUT /api/kv/:key
        // router.put("/api/kv/:key",  middleware!{ |request, response|
            // let entity_set = request.json_as::<EntitySet>().unwrap();
            // match request.param("key") {
            //     Some(key) => {
            //         format!("{:?}", entity_set.value)
            //     },
            //     _ => format!("success")
            // }
        //     "error"
        // });

//        let mut data = HashMap::<&str, &str>::new();
//        data.insert("name", "Alex");

        //router.get("/api/**", middleware!("You call API"));
        router
    }

    pub fn run(&self) {
        let mut server = Nickel::with_options(Options::default().output_on_listen(false));

        server.utilize(handler_logger);

        server.utilize(StaticFilesHandler::new("assets/"));
        server.utilize(StaticFilesHandler::new("webui/dist"));

        server.utilize(self.router_api());
        server.utilize(self.router_webui());

//        let custom_handler: fn(&self, &mut NickelError, &mut Request) -> Action = &self.handler_error_404;
//        server.handle_error(custom_handler);

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

        if self.use_tls {
            // TODO: Implement HTTPS (https://github.com/nickel-org/nickel.rs/blob/master/examples/https.rs)
//            server.listen_https()
        }
    }

    fn handler_error_404<'a>(&self, err: &mut NickelError, _request: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                // TODO: display vuejs error page
                res.write_all(b"404 Not Found").expect("Unable to write in the stream");
                return Halt(());
            }
        }
        Continue(())
    }

    pub fn dispose(&self) {
        self.log(LogLevel::Information, "Stopping the Lucid server", None);
    }
}