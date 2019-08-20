use std::io::Write;
use std::collections::HashMap;
use nickel::status::StatusCode::NotFound;
// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, NickelError, Action, Halt, Continue};
use nickel::*;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;

include!("../logger.rs");

fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    log_event(LucidError::Information, format!("{}", request.origin.uri), format!("{}", request.origin.method));
    response.next_middleware()
}

fn handler_error_404<'a>(err: &mut NickelError, _request: &mut Request) -> Action {
    if let Some(ref mut res) = err.stream {
        if res.status() == NotFound {
            // TODO: display vuejs error page
            log_event(LucidError::Warning, format!("{}", _request.origin.uri), format!("{}", _request.origin.method));
            res.write_all(b"404 Not Found").expect("Unable to write in the stream");
            return Halt(())
        }
    }
    Continue(())
}

fn handler_vuejs<'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "Alex");
    res.render("webui/dist/index.tpl", &data)
}

pub struct Server {
    endpoint: String,
    configuration_file: Option<String>
}

impl Server {
    pub fn new() -> Server {
        Server {
            endpoint: String::from("127.0.0.1:7021"),
            configuration_file: None
        }
    }

    pub fn set_configuration_file(&mut self, _configuration_file: String)
    {
        self.configuration_file = Some(_configuration_file);
    }

    pub fn has_configuration_file(&mut self) -> bool {
        self.configuration_file.is_some()
    }

    pub fn load_configuration(&mut self) {
        if let Some(configuration_file) = &self.configuration_file {
            if let Ok(mut file) = File::open(configuration_file) {
                let mut lucid_yml = String::new();
                file.read_to_string(&mut lucid_yml).unwrap();
                let docs = YamlLoader::load_from_str(lucid_yml.as_str()).unwrap();
                let doc = &docs[0];
                match doc["endpoint"].as_str()
                {
                    Some(endpoint) => { self.endpoint = String::from(endpoint); }
                    None => ()
                }
            }
        }
    }
    
    fn router_webui(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.get("/", handler_vuejs);
        router
    }
    
    fn router_api(&self) -> nickel::Router {
        let mut router = Nickel::router();
        router.post("/api/**", middleware!("You call API [post]"));
        router.get("/api/**", middleware!("You call API"));
        router
    }

    pub fn run(&self) -> Result<ListeningServer, Box<dyn std::error::Error>> {
        // TODO: move into struct
        let mut daemon = Nickel::new();
        
        daemon.options = Options::default().output_on_listen(false);

        daemon.utilize(logger);

        daemon.utilize(StaticFilesHandler::new("assets/"));
        daemon.utilize(StaticFilesHandler::new("webui/dist"));

        daemon.utilize(self.router_api());
        daemon.utilize(self.router_webui());

        let custom_handler: fn(&mut NickelError, &mut Request) -> Action = handler_error_404;
        daemon.handle_error(custom_handler);

        // TODO: Implement HTTPS (https://github.com/nickel-org/nickel.rs/blob/master/examples/https.rs)
        daemon.listen(self.endpoint.to_owned())
        // daemon.listen(("0.00.0", self.where_to_bind()))
    }
}
