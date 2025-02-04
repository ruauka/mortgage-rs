# Rust as the base image
FROM rust:1.75.0-slim-bullseye AS builder

# fix rust http openssl bug
RUN apt update
RUN apt install libssl-dev
RUN apt install pkg-config -y

# Create a new empty shell project
RUN USER=root cargo new --bin mg
WORKDIR /mg

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm -f ./target/release/deps/mg*
RUN cargo build --release

# The final base image
FROM debian:12.5-slim
WORKDIR /app
# Copy from the previous build
COPY --from=builder /mg/target/release/mg .

# Run the binary
CMD ["./mg", "--host", "0.0.0.0", "--port", "8080"]
