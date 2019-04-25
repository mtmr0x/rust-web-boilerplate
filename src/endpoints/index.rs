extern crate iron;
extern crate router;

use iron::{Request, Response, IronResult};
use iron::status;

pub fn index_handler(_req: &mut Request) -> IronResult<Response> {
    info!("responding index");
    Ok(Response::with((status::Ok, "Hello world".to_string())))
}

