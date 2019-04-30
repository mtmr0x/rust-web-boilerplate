#!/bin/sh

source .env

LEVEL_VERBOSITY=$LEVEL_VERBOSITY \
SERVER_PORT=$SERVER_PORT \
  ~/.cargo/bin/cargo run

