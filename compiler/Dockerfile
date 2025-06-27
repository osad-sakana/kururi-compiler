# マルチステージビルドでイメージサイズを最適化
FROM rust:1.83-slim as builder

WORKDIR /app

# 依存関係のキャッシュを最適化するため先にCargo.tomlをコピー
COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "pub fn lib_fn() {}" > src/lib.rs && cargo build --release && rm -rf src

# ソースコードをコピーしてビルド
COPY src src
RUN cargo build --release

# 実行用の軽量イメージ
FROM debian:bookworm-slim

# SSL証明書とタイムゾーンのインストール（HTTPSリクエスト用）
RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# 実行ユーザーを作成（セキュリティ向上）
RUN useradd --create-home --shell /bin/bash app

WORKDIR /app
USER app

# ビルドしたバイナリのみをコピー
COPY --from=builder /app/target/release/kururi-compiler .

EXPOSE 8080

CMD ["./kururi-compiler"]