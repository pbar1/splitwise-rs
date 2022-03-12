# splitwise-rs

[![CI](https://github.com/pbar1/splitwise-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/pbar1/splitwise-rs/actions/workflows/ci.yml)
[![Docs.rs](https://img.shields.io/docsrs/splitwise)](https://docs.rs/splitwise)
[![Crates.io](https://img.shields.io/crates/v/splitwise.svg)](https://crates.io/crates/splitwise)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.56.1-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.56.1)

Splitwise SDK for Rust

## Usage

The default Splitwise client reads an API key from the environment variable `SPLITWISE_API_KEY`.
API keys can be generated in the [Splitwise developer portal](https://secure.splitwise.com/apps).

```rust
#[tokio::main]
async fn main() {
    let client = splitwise::client::Client::default();

    let user = client.users().get_current_user().await.unwrap();

    println!("Current user: {:#?}", user)
}
```

## Roadmap

- [ ] Support for sync and async via crate features
- [ ] Make HTTP client generic, with default implementations
- [ ] Handle Splitwise API versioning
- [ ] More robust error handling
- [ ] Cut down on some `Request` and `Response` types in favor of function parameters
