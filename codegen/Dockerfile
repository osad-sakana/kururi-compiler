FROM rust:1.83.0-slim

WORKDIR /app
COPY . .

RUN cargo build --release
CMD ["/app/target/release/codegen"]
