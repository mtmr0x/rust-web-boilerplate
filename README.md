# Rust web server example

It's a work in progress web server in Rust I intend to work on for the past days until I get a result which has the basic set up for creating small or big applications starting from this codebase.

## Installation

Install Rust

Go to https://www.rust-lang.org/tools/install and check installation methods

Use Cargo to compile and run with this simple command:

```shell
cargo run
```

You can pass `SERVER_PORT` environment variable to the process, otherwise the application runs by default at `http://localhost:5000`

## To-do list

- [x] Web framework (Iron): https://docs.rs/iron/0.6.0/iron/index.html
- [x] Routing: https://docs.rs/router/0.6.0/router/index.html
- [ ] Get environment variables from profile
- [ ] Have a sexy log tool great for instrumentation
- [ ] Implement GraphQL

