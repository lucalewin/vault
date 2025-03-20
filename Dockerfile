# Builder Stage
FROM debian:bullseye-slim AS builder

# Install dependencies and clean up in one layer
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl unzip ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set environment variable
ENV DATABASE_URL=postgresql://postgres:abcd@server.local:5432/vault

# Download and extract the release
RUN curl -L -o release.zip https://github.com/lucalewin/vault/releases/latest/download/vault-aarch64-unknown-linux-gnu.zip && \
    unzip vault-aarch64-unknown-linux-gnu.zip -d release && \
    rm release.zip

# Runner Stage
FROM debian:bullseye-slim AS runner

# Set working directory
WORKDIR /app

# Copy only the necessary files from the builder stage
COPY --from=builder /release/release/vault ./vault
COPY --from=builder /release/release/static ./static

# Set entry point
ENTRYPOINT ["./vault"]

# # FROM debian:latest AS builder


# # WORKDIR /app

# # RUN ls -la ./release

# FROM debian:bullseye-slim AS builder

# RUN apt-get -y update; apt-get -y install curl unzip
# # RUN install_packages ca-certificates curl unzip

# ENV DATABASE_URL=postgresql://postgres:abcd@server.local:5432/vault
# RUN curl -L -o release.zip https://github.com/lucalewin/vault/releases/latest/download/release.zip
# RUN unzip release.zip -d release
# # RUN rm -rf /release

# FROM debian:bullseye-slim AS runner
# WORKDIR /app
# COPY --from=builder /release/release/vault ./vault
# COPY --from=builder /release/release/static ./static

# # FROM alpine AS runner
# # COPY --from=builder /release/release/vault ./vault
# # COPY --from=builder /release/release/static ./static

# ENTRYPOINT [ "./vault" ]

# # # build frontend
# # FROM oven/bun:latest AS frontend
# # WORKDIR /app/frontend

# # # RUN apt update && apt install -y procps
# # #&& rm -rf /var/lib/apt/lists/*

# # COPY ./frontend ./
# # RUN bun install --frozen-lockfile
# # RUN bun run build

# # # build backend

# # FROM rust:alpine AS backend-builder
# # RUN apk add --no-cache build-base musl-dev openssl-dev openssl
# # WORKDIR /app/backend

# # # install dependencies
# # COPY Cargo.toml Cargo.lock ./
# # RUN mkdir src && echo "fn main() {}" > src/main.rs
# # RUN cargo fetch
# # RUN cargo build --release
# # RUN rm src/main.rs

# # # actual build
# # COPY ./src ./src
# # COPY ./migrations ./migrations
# # COPY ./.sqlx ./.sqlx
# # RUN SQLX_OFFLINE=true cargo build --release

# # FROM chef AS planner
# # COPY . .
# # RUN cargo chef prepare --recipe-path recipe.json

# # FROM chef AS backend-builder 
# # COPY --from=planner /app/backend/recipe.json recipe.json
# # # Build dependencies - this is the caching Docker layer!
# # RUN cargo chef cook --release --recipe-path recipe.json
# # # Build application
# # COPY . .
# # RUN SQLX_OFFLINE=true cargo build --release

# # FROM rust:latest AS backend-builder
# # WORKDIR /app/backend

# # COPY ./Cargo.toml ./Cargo.toml
# # COPY ./Cargo.lock ./Cargo.lock
# # COPY ./src ./src
# # COPY ./migrations ./migrations
# # COPY ./.sqlx ./.sqlx
# # RUN SQLX_OFFLINE=true cargo build --release

# # final image
# # FROM alpine AS runner
# # COPY --from=frontend /app/frontend/dist ./static
# # COPY --from=backend-builder /app/backend/target/release/vault ./vault

# # CMD ["./vault"]
