FROM rust:1.67-slim AS builder

WORKDIR /app
COPY backend .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/rustchain-backend /app/rustchain
COPY backend/migrations /app/migrations

CMD ["/app/rustchain"]
