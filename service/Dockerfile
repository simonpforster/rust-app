FROM rust:1.76.0-buster as builder
WORKDIR /rust-app
ADD .. .

RUN cargo install --path service


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/service /usr/local/bin/service
COPY --from=builder /rust-app/service/resources /service/resources

ENV RUST_BACKTRACE=full

EXPOSE 8080

CMD ["service"]