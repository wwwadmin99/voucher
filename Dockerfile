FROM node:20-slim AS frontend-build
WORKDIR /app/frontend
RUN corepack enable
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY frontend/ ./
RUN pnpm build

FROM rust:1-slim AS backend-build
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/api ./backend/api
RUN cargo build --release -p voucher-api

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-build /app/target/release/voucher-api ./voucher-api
COPY --from=frontend-build /app/frontend/dist ./static
ENV STATIC_DIR=/app/static
EXPOSE 8080
ENTRYPOINT ["./voucher-api"]
