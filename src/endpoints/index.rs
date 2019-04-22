pub mod index_handler {
    extern crate iron;
    extern crate router;

    use iron::{Request, Response, IronResult};
    use iron::status;
    use router::Router;

    pub fn handler(req: &mut Request) -> IronResult<Response> {
        println!("Responding index");

        Ok(Response::with((status::Ok, "Hello world".to_string())))
    }
}

