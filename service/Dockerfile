FROM rust:1.76.0-buster as builder
WORKDIR /service
COPY .. .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/service /usr/local/bin/service
COPY --from=builder /service/resources /service/resources

EXPOSE 8080

CMD ["service"]