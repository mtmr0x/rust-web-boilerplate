extern crate rustc_version;
extern crate iron;
extern crate router;
extern crate fern;
#[macro_use]
extern crate log;

use std::io::{self, Write};
use std::process::exit;
use rustc_version::{version, Version};

use iron::{Iron};

mod endpoints;
mod config;
use config::config::{server_port, level_verbosity, rustc_version};
mod logger;
use logger::logger::setup_logger;

mod routes;
use routes::routes::app_routes;

fn main() {
    if version().unwrap() != Version::parse(&rustc_version()).unwrap() {
        writeln!(&mut io::stderr(), "This crate requires rustc version {} but found {}", rustc_version(), version().unwrap()).unwrap();
        exit(1);
    }

    setup_logger(level_verbosity()).expect("Could not load logger");
    info!("Loading server");

    let server_url:String = format!("localhost:{}", server_port());

    info!("server is running at {}", server_url);
    Iron::new(app_routes()).http(server_url).unwrap();
}

