use std::io::Write;
use std::collections::HashMap;
use nickel::status::StatusCode::NotFound;
// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, NickelError, Action, Halt, Continue};
use nickel::*;

include!("../logger.rs");

fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    log_event(format!("{}", request.origin.uri));
    response.next_middleware()
}

fn handler_error_404<'a>(err: &mut NickelError, _request: &mut Request) -> Action {
    if let Some(ref mut res) = err.stream {
        if res.status() == NotFound {
            // TODO: display vuejs error page
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
    port: i32,
}

impl Server {
    pub fn new() -> Server {
        Server {
            port: 7221
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
        daemon.listen(self.where_to_bind())
        // daemon.listen(("0.00.0", self.where_to_bind()))
    }

    fn where_to_bind(&self) -> String
    {
        // env::var("PORT").unwrap_or("7221".to_string()).parse().unwrap()
        // TODO: implement configuration 
        return format!("127.0.0.1:{}", self.port);
    }
}
