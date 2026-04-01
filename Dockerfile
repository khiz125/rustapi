# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.88
ARG APP_NAME=rustapi

# --- cargo-chef ベースステージ ---
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app

# --- planner: 依存関係キャッシュ用 recipe 生成 ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- builder: 依存関係ビルド + アプリ本体 ---
FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json

# キャッシュマウント付きで依存ビルド
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# ソースコピーして本体ビルド
COPY . .
# offline mode で sqlx データがあれば DB 不要
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --bin rustapi && \
    cp ./target/release/rustapi /bin/server

# --- runtime: distroless ---
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=builder /bin/server /app/
WORKDIR /app
EXPOSE 8000
ENTRYPOINT ["/app/server"]
