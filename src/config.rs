pub mod config {
    use std::env;

    fn get_env_var(key: String, default: String) -> String {
        println!("get env var {}", key);
        match env::var(key) {
            Ok(val) => val.to_string(),
            Err(_e) => default,
        }
    }

    pub fn server_port() -> String {
        get_env_var("SERVER_PORT".to_string(), "5000".to_string())
    }

//    struct Database {
//        name: String,
//        host: String,
//        port: String,
//        user: String,
//        password: String,
//    }
//
//    pub fn database() -> Database {
//        let name:String = match env::var("DATABASE_NAME".to_string()) {
//            Ok(val) => val.to_string(),
//            Err(_e) => "4000".to_string(),
//        };
//    }
}

