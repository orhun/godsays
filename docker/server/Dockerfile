FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --locked --release --verbose --features server --bin godsays-server

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/godsays-server .
EXPOSE 3000
USER 1000:1000
CMD ["./godsays-server"]
