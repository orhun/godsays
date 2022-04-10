FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --locked --release --verbose
RUN cp /home/rust/src/target/x86_64-unknown-linux-musl/release/godsays .
RUN strip /home/rust/src/godsays

FROM scratch
COPY --from=builder /home/rust/src/godsays .
USER 1000:1000
CMD ["./godsays"]
