pub mod routes {
    extern crate router;
    use router::Router;

    use endpoints::index::index_handler;

    pub fn app_routes() -> Router {
        let mut router = Router::new();

        router.get("/", index_handler, "index");

        router
    }
}

