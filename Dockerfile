FROM docker.io/rust:alpine as builder

RUN apk update && apk add --no-cache build-base

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new oldpaper

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/oldpaper/

# Set the working directory
WORKDIR /usr/src/oldpaper

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/oldpaper/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/oldpaper/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM docker.io/alpine:latest

COPY --from=builder /usr/src/oldpaper/target/x86_64-unknown-linux-musl/release/oldpaper /usr/local/bin

# Run the application
CMD ["/usr/local/bin/oldpaper"]
