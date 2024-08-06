# Rust app

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
