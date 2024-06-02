ARG BASE_IMAGE=rust:1.75.0-slim-buster

FROM $BASE_IMAGE as planner
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"
WORKDIR /workspace
RUN cargo install cargo-chef --version 0.1.64 --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM $BASE_IMAGE as cacher
RUN apt update \
    && apt install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
RUN cargo install cargo-chef --version 0.1.64 --locked
RUN cargo install bunyan

COPY --from=planner /workspace/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM cacher as builder
WORKDIR /workspace
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /workspace/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"

COPY static static
COPY settings settings
COPY --from=builder /bin/sh /bin/sh
COPY --from=builder /workspace/target/release/app /
COPY --from=cacher /usr/local/cargo/bin/bunyan /usr/local/bin/bunyan

ENV APP_PROFILE prod
ENV APP_DB__HOST postgres-db-1
ENV RUST_LOG info
ENV APP_SERVER__PORT 8000

EXPOSE $APP_SERVER__PORT

ENTRYPOINT ["sh", "-c", "./app | bunyan"]