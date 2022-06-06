# syntax = docker/dockerfile:1.2

# Container configuration
FROM rust:1.61-alpine AS base

# Metadata
LABEL maintainer="lukedavia@icloud.com"
WORKDIR /var/www

# https://stackoverflow.com/questions/30624829
# RUN apt install openssl-dev

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev openssl-dev

# Copy over the source.
COPY . .

# Install dependencies and compile acars.
RUN cargo install --path .
RUN cargo build --target=x86_64-unknown-linux-musl --package=acars --bin=acars --release

# ----------------------------------------------------------------------------------------------------------------------
# Include only the binary in the final build
FROM alpine:3.16

WORKDIR /var/www
COPY --from=base /var/www/target/x86_64-unknown-linux-musl/release /usr/local/bin

ENTRYPOINT ["acars"]