# syntax=docker/dockerfile:1

FROM rust:latest AS builder

ENV SQLX_OFFLINE=true 
WORKDIR /build
COPY . .
RUN \
    --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && cp target/release/todolist-api /usr/local/bin/api

FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=builder /usr/local/bin/api ./

CMD ["./api"]
