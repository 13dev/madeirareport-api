# syntax=docker/dockerfile:1
# build stage
FROM rust:latest as planner
WORKDIR /workspace
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest as builder
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"
WORKDIR /workspace
RUN apt-get update && apt-get install lld clang -y
COPY --from=planner /workspace/recipe.json recipe.json
COPY . .
RUN cargo install bunyan
RUN cargo chef cook --release --recipe-path recipe.json

# deploy stage
FROM debian:bookworm-slim
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"
RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean
WORKDIR /workspace
COPY static static
COPY settings settings
COPY --from=builder /usr/local/cargo/bin/bunyan /usr/local/bin/bunyan
COPY --from=builder /workspace/target/release/app .
EXPOSE 8081
ENV APP_PROFILE prod
ENV RUST_LOG info
ENTRYPOINT ["sh", "-c", "./app | bunyan"]