use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod comments;
pub mod expenses;
pub mod friends;
pub mod groups;
pub mod notifications;
pub mod other;
pub mod users;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorUnauthorized {
    pub error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorForbiddenOrNotFound {
    pub errors: ErrorsBase,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorsBase {
    pub base: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Success {
    pub success: bool,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub original: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub xlarge: Option<String>,
    pub xxlarge: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub currency_code: Option<String>,
    pub amount: Option<String>,
}

/// Debt relationship between two users.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debt {
    /// ID of the user who owes money.
    pub from: Option<i64>,

    /// ID of the user who paid money.
    pub to: Option<i64>,

    /// Decimal amount as a string with 2 decimal places.
    pub amount: Option<String>,

    /// A currency code. Must be in the list from `get_currencies`.
    pub currency_code: Option<String>,
}
