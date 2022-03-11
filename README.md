# splitwise-rs

[![Crates.io](https://img.shields.io/crates/v/splitwise.svg)](https://crates.io/crates/splitwise)
[![Docs.rs](https://img.shields.io/docsrs/splitwise)](https://docs.rs/splitwise)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.56.1-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.56.1)

Splitwise SDK for Rust

## Progress

| API            | Implementations | Integration Tests | Documentation |
|----------------|-----------------|-------------------|---------------|
| Authentication | 1/2             | 1/2               |               |
| Users          | Complete        | Complete          | Complete      |
| Groups         | Complete        | Complete          | Complete      |
| Friends        | Complete        | Complete          |               |
| Expenses       | Complete        | Complete          |               |
| Comments       | Complete        | Complete          |               |
| Notifications  | Complete        | Complete          |               |
| Other          | Complete        | Complete          |               |

## TODO

- Cargo features for sync and async
- Make HTTP client generic and pluggable
- Document everything
- Handle Splitwise API versioning
- All of the `.error` and `.errors` properties should be handled more robustly
- Disambiguate API names - ie, prefer `list_users` over `get_users`, as `get_user` also exists
- Cut down on some superfluous `*Request` and `*Response` types in favor of function params
