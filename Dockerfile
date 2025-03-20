# Builder Stage
FROM debian:bullseye-slim AS builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl unzip ca-certificates

# Download and extract the release
RUN curl -L -o release.zip https://github.com/lucalewin/vault/releases/latest/download/vault-aarch64-unknown-linux-gnu.zip && \
    unzip release.zip -d release && \
    rm release.zip

# Runner Stage
FROM debian:bullseye-slim AS runner
WORKDIR /app

# Copy only the necessary files from the builder stage
COPY --from=builder /release/vault ./vault
COPY --from=builder /release/static ./static

ENTRYPOINT ["./vault"]