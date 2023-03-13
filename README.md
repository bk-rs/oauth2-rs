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

providers/oauth2-doorkeeper
providers/*

oauth2-signin
