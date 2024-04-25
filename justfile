#!/usr/bin/env just --justfile

docker command='up' args='':
    podman compose -p "madeirareport-api" --env-file ".env.dev" {{command}} -d {{args}}

docker-logs service='':
    podman compose -p "madeirareport-api" --env-file ".env.dev" logs {{service}}

release:
  cargo build --release

watch:
  RUSTFLAGS="-Awarnings" cargo watch -x 'run --bin app'

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1