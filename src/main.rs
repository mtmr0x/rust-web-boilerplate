extern crate iron;
extern crate router;
extern crate fern;
#[macro_use]
extern crate log;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;

mod endpoints;
mod config;
mod logger { pub mod logger };

use logger::logger::setup_logger;

use endpoints::index::index_handler::handler;
use config::config::server_port;

fn main() {
    setup_logger().expect("Could not load logger");
    info!("Loading server");

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", query_handler, "query");
    let server_url:String = format!("localhost:{}", server_port());

    info!("server is running at {}", server_url);
    Iron::new(router).http(server_url).unwrap();

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        println!("Routing: get from {}", req.url);
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}

