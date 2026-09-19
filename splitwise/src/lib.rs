//! Splitwise SDK for Rust
//!
//! ## Usage
//!
//! The default Splitwise client reads an API key from the environment variable `SPLITWISE_API_KEY`.
//! API keys can be generated in the [Splitwise developer portal](https://secure.splitwise.com/apps).
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() {
//!     let client = splitwise::client::Client::default();
//!
//!     let user = client.users().get_current_user().await.unwrap();
//!
//!     println!("Current user: {:#?}", user)
//! }
//! ```

pub mod client;
pub mod model;
