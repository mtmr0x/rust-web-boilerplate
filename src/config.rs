pub mod config {
    use std::env;
    use std::option::Option;

    fn get_env_var(key: String) -> Option<String> {
        match env::var(key) {
            Ok(val) => Some(val),
            Err(_e) => None,
        }
    }

    pub fn server_port() -> String {
        get_env_var("SERVER_PORT".to_string()).unwrap()
    }
}

