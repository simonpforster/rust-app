# Rust app

[![PR Check](https://github.com/simonpforster/rust-app/actions/workflows/service-ci.yml/badge.svg?branch=master)](https://github.com/simonpforster/rust-app/actions/workflows/service-ci.yml)

## Service

``cargo run`` or ``docker compose up -d``

## Testing

### Unit test

``cargo test --package service``

### Functional test

``docker compose up -d``

``cargo test --package functional-test`` while service is running.

### Load test

``docker compose up -d``

``cargo run --bin load-test`` while service is running.
