# Rust app

[![PR Check](https://github.com/simonpforster/rust-app/actions/workflows/on-pr.yml/badge.svg?branch=master)](https://github.com/simonpforster/rust-app/actions/workflows/on-pr.yml)

## Service

``cargo run`` or ``docker compose up -d``

## Testing

### Unit test

``cargo test --package service``

### Functional test

``docker compose up -d``

``cargo test --package function-test`` while service is running.

### Load test

``docker compose up -d``

``cargo run --bin load-test`` while service is running.
