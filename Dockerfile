FROM rust:1.88.0 AS builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

ENV RUSTFLAGS="-C target-feature=+crt-static"
WORKDIR /app

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /backend

EXPOSE 3000
ENTRYPOINT ["/backend"]