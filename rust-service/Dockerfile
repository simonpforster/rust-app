FROM rust:1.76.0-buster as builder
WORKDIR /app
COPY .. .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rust-service /usr/local/bin/rust-service

EXPOSE 8080

CMD ["rust-service"]