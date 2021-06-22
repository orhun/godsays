FROM rust:1.52-slim-buster as builder
WORKDIR /src
COPY Cargo.toml Cargo.toml
RUN mkdir -p src/bin/
RUN echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN echo "fn main() {println!(\"failed to build server\")}" > src/bin/server.rs
RUN cargo build --release --verbose \
    --features server --bin godsays-server
RUN rm -f target/release/deps/godsays*
COPY . .
RUN cargo build --locked --release \
    --verbose --features server --bin godsays-server
RUN mkdir -p build-out && cp target/release/godsays-server build-out/

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /src/build-out/godsays-server .
EXPOSE 3000
USER nobody
CMD ["./godsays-server"]
