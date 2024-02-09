# base stage for dependencies
FROM rust:latest as dependencies
LABEL org.opencontainers.image.source = "https://github.com/13dev/madeirareport-api"

WORKDIR /workspace

RUN apt-get update && apt-get install lld clang -y

# Install cargo chef
RUN cargo install cargo-chef --version 0.1.33

COPY . .

# Preheat the dependency cache
RUN cargo chef prepare --recipe-path recipe.json

# build stage
FROM dependencies as builder

# Copy over the pre-built dependencies
COPY --from=dependencies /workspace/recipe.json recipe.json

# Build the project
RUN cargo chef cook --release --recipe-path recipe.json

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
