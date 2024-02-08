# build stage
FROM rust:latest as builder
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"

WORKDIR /workspace

RUN apt-get update && apt-get install lld clang -y

COPY . .

RUN cargo install bunyan
RUN cargo build --release

# deploy stage
FROM debian:bookworm-slim
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean

# create workspace directory
WORKDIR /workspace

COPY static static

COPY settings settings

# copy binary and configuration files
COPY --from=builder /usr/local/cargo/bin/bunyan /usr/local/bin/bunyan
COPY --from=builder /workspace/target/release/app .

# expose port
EXPOSE 8081

ENV APP_PROFILE prod

ENV RUST_LOG info

# run the binary
ENTRYPOINT ["sh", "-c", "./app | bunyan"]