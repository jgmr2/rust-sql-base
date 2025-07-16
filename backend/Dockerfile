# Etapa 1: Compilación
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev ca-certificates
WORKDIR /app
COPY Cargo.toml .
COPY src/ src/
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/backend

# Etapa 2: Imagen final ultra minimalista
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /backend
EXPOSE 80
ENTRYPOINT ["/backend"]
