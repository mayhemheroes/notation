FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /notation
WORKDIR /notation/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /notation/fuzz/target/x86_64-unknown-linux-gnu/release/notation-parser-fuzz /