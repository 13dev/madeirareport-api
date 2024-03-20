#!/usr/bin/env just --justfile

docker command='up' args='':
    docker-compose --verbose -p "madeirareport-api" {{command}} -d {{args}}

release:
  cargo build --release


build:
  cargo build

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1