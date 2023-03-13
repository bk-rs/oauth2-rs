## Dev

```
cargo clippy --all-features --tests --examples -- -D clippy::all
cargo +nightly clippy --all-features --tests --examples -- -D clippy::all

cargo fmt -- --check

cargo test-all-features -- --nocapture
```

## Publish providers

oauth2-core

oauth2-client

oauth2-signin

providers/oauth2-doorkeeper
providers/*
