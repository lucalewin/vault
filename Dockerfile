# build frontend
FROM oven/bun:latest AS frontend
WORKDIR /app/frontend

# RUN apt update && apt install -y procps
#&& rm -rf /var/lib/apt/lists/*

COPY ./frontend ./
RUN bun install --frozen-lockfile
RUN bun run build

# build backend

FROM rust:alpine AS backend-builder
RUN apk add --no-cache build-base musl-dev openssl-dev openssl
WORKDIR /app/backend

# install dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# actual build
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
RUN SQLX_OFFLINE=true cargo build --release

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS backend-builder 
# COPY --from=planner /app/backend/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY . .
# RUN SQLX_OFFLINE=true cargo build --release

# FROM rust:latest AS backend-builder
# WORKDIR /app/backend

# COPY ./Cargo.toml ./Cargo.toml
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./src ./src
# COPY ./migrations ./migrations
# COPY ./.sqlx ./.sqlx
# RUN SQLX_OFFLINE=true cargo build --release

# final image
FROM alpine AS runner
COPY --from=frontend /app/frontend/dist ./static
COPY --from=backend-builder /app/backend/target/release/vault ./vault

CMD ["./vault"]
