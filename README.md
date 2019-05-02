# Rust web server example

It's a work in progress web server in Rust I intend to work on for the next days until I get a result which has the basic set up for creating small or big applications starting from this codebase.

## Installation

Install Rust

Go to https://www.rust-lang.org/tools/install and check installation methods

## Set up

You'll have to load in memory environment variables present in `start.sh` file.

## Run

Execute the `start.sh` script located in the this project:

```shell
./start.sh
```

It will add environment variables to the process and use cargo binaries to compile and run.

## To-do list

- [x] Web framework (Iron): https://docs.rs/iron/0.6.0/iron/index.html
- [x] Routing: https://docs.rs/router/0.6.0/router/index.html
- [x] Get environment variables from profile
- [x] Have a sexy log tool great for instrumentation
- [ ] Document its folder structure and how to run properly
- [ ] Implement GraphQL

