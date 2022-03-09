# splitwise-rs

Splitwise SDK for Rust

## Progress

| API            | Implementations | Integration Tests |
|----------------|-----------------|-------------------|
| Authentication | 1/2             | 1/2               |
| Users          | Complete        | Complete          |
| Groups         | Complete        | Complete          |
| Friends        | Complete        | 2/5               |
| Expenses       | Complete        | Complete          |
| Comments       | Complete        | Complete          |
| Notifications  | Complete        | Complete          |
| Other          | Complete        | Complete          |

## TODO

- Cargo features for sync and async
- Make HTTP client generic and pluggable
- Document everything
- Handle Splitwise API versioning
- All of the `.error` and `.errors` properties should be handled more robustly
- Disambiguate API names - ie, prefer `list_users` over `get_users`, as `get_user` also exists
- Cut down on some superfluous `*Request` and `*Response` types in favor of function params
