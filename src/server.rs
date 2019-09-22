use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::Ipv4Addr;
use std::str::FromStr;

use nickel::{*, Action, Continue, Halt, HttpRouter, Middleware, MiddlewareResult, Nickel, NickelError, Options, Request, Response, StaticFilesHandler};
use nickel::status::StatusCode::NotFound;
use yaml_rust::YamlLoader;

use crate::configuration::Configuration;
use crate::logger::{Logger, LogLevel};

pub struct Server {
    endpoint: String,
    use_tls: bool,
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

impl Server
{
    pub fn default() -> Server
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
        router.get("/**", handler_vuejs);
        router
    }

    fn router_api(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.post("/api/**", middleware!("You call API [post]"));
        router.get("/api/**", middleware!("You call API"));
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