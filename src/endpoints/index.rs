pub mod index_handler {
    extern crate iron;
    extern crate router;

    use iron::{Request, Response, IronResult};
    use iron::status;

    pub fn handler(_req: &mut Request) -> IronResult<Response> {
        println!("Responding index");

        Ok(Response::with((status::Ok, "Hello world".to_string())))
    }
}

