FROM rust:1.76.0-bookworm as chef

RUN cargo install cargo-chef
WORKDIR /rust-app
ADD .. .

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /rust-app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin service

FROM rust:1.76.0-bookworm as runner

COPY --from=builder /rust-app/target/release/service /usr/local/bin
COPY --from=builder /rust-app/service/resources /service/resources

ENV RUST_BACKTRACE=full

EXPOSE 8080

CMD ["/usr/local/bin/service"]
