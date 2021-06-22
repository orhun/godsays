FROM rust:1.52-slim-buster as builder
WORKDIR /src
COPY Cargo.toml Cargo.toml
RUN mkdir src/ && echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN cargo build --release --verbose
RUN rm -f target/release/deps/godsays*
COPY . .
RUN cargo build --locked --release --verbose
RUN mkdir -p build-out && cp target/release/godsays build-out/

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /src/build-out/godsays .
USER nobody
CMD ["./godsays"]
