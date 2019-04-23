#!/bin/sh

source .env

SERVER_PORT=$SERVER_PORT \
  ~/.cargo/bin/cargo run

