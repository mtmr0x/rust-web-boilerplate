# Rust Web Server Boilerplate

Start developing your Rust server based in this simple set up.

 - [Browse features](#features-list) to understand better what you can find in here;
 - [Check table of contents](#table-of-contents) for full documentation.

The following instructions are for MacOS or Linux.

If something doesn't work, open an issue or open a Pull Request following [contribution guidelines](https://github.com/mtmr0x/rust-web-boilerplate/blob/master/CONTRIBUTING.md);

## Quick overview

```sh
# copy env sample file to .env file
cp ./.env.sample ./.env

# open it and replace its values
vim ./.env
```

Follow [env files section](#environment-variables) for understanding it.

Then start the server using the `start.sh` script.

```sh
./start.sh # make sure you gave permissions to it
```

## Table of contents

- [Installation](#installation)
- [Set up](#set-up)
    - [Environment variables](#environment-variables)
- [Run](#run)
- [Features](#features)
    - [Logger](#logger)
    - [Configuration](#configuration)
    - [Routing](#routing)

### Installation

Install Rust

Go to https://www.rust-lang.org/tools/install and check installation methods

> Note: this project is set with version 1.34.1 and tested with 1.29.0 and worked pretty well. You can define your version of Rust at your [environment variables](#environment-variables).

### Set up

#### Environment variables

For properly run this project, you will have to set some environment variables. Everything you need to set up is located in [.env.sample file](https://github.com/mtmr0x/rust-web-boilerplate/blob/master/.env.sample).

> Every update in your `.env` file you must add the new variable to the process at your start up script before running it. Open and edit the `start.sh` file present in the root directory of this project.

### Run

Execute the `start.sh` script located in the this project:

```shell
./start.sh
```

### Features

#### Logger

Logger configurations depends on `LEVEL_VERBOSITY` environment variable for deciding what levels of logs can be printed. `LEVEL_VERBOSITY` documentation and usage is present in `.env.sample` file.

**stdout log**:

The application logs stdout for instrumentation and its format is present in `src/logger/logger.rs` file, logging hour, target, level and message.

**output file log**:

The file output is set as `output.log` file, that will be present in the root directory as it has logs. It prints the same format of stdout logs with addition of Year, Month and Day of that output before the time of it.

#### Configuration

This application tries to follow _configuration over convention_. All types of configurations that is not Rust convention is placed as environment variable and all of them must be set, otherwise the application will fail to start.

You can find all necessary environment variables documented for this project inside `.env.sample` file.

#### Routing

This project is meant to be easy for people coming from NodeJS and that said the closest way to get there was to find a routing declaration library and framework that could make web development similar to ExpressJS or Koa. [Iron](https://docs.rs/iron/0.6.0/iron/index.html) and [Router](https://docs.rs/router/0.6.0/router/index.html) did the job.


## To-do list

- [x] Web framework (Iron): https://docs.rs/iron/0.6.0/iron/index.html
- [x] Routing: https://docs.rs/router/0.6.0/router/index.html
- [x] Get environment variables from profile
- [x] Have a sexy log tool great for instrumentation
- [x] Document how to run
- [ ] Implement GraphQL

