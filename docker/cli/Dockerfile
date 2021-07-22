FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --locked --release --verbose

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/godsays .
USER 1000:1000
CMD ["./godsays"]
