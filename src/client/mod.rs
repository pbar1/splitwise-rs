#![allow(clippy::module_inception)]

pub(crate) mod authentication;
pub(crate) mod client;
pub(crate) mod comments;
pub(crate) mod expenses;
pub(crate) mod friends;
pub(crate) mod groups;
pub(crate) mod notifications;
pub(crate) mod other;
pub(crate) mod users;

pub use authentication::*;
pub use client::*;
pub use comments::*;
pub use expenses::*;
pub use friends::*;
pub use groups::*;
pub use notifications::*;
pub use other::*;
pub use users::*;
