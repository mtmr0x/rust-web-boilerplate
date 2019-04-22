extern crate iron;
extern crate router;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;

mod endpoints;
mod config;

use endpoints::index::index_handler::handler;
use config::config::server_port;

fn main() {
    println!("Loading server");

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", query_handler, "query");
    let server_url:String = format!("localhost:{}", server_port());

    println!("server is running at {}", server_url);
    Iron::new(router).http(server_url).unwrap();

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        println!("Routing: get from {}", req.url);
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}

