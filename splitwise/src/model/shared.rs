use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// Unauthorized error.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorUnauthorized {
    /// Error that occurred during this request.
    pub error: String,
}

/// Forbidden or not found error.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorForbiddenOrNotFound {
    /// Errors that occurred during this request.
    pub errors: ErrorsBase,
}

/// Error wrapper.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorsBase {
    /// List of errors that occurred.
    pub base: Vec<String>,
}

/// Success wrapper.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Success {
    /// Whether the request succeeded.
    pub success: bool,

    /// List of errors that occurred.
    pub errors: Option<HashMap<String, Vec<String>>>,
}

/// Splitwise image in various sizes.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    /// Original image size.
    pub original: Option<String>,

    /// Small image size.
    pub small: Option<String>,

    /// Medium image size.
    pub medium: Option<String>,

    /// Large image size.
    pub large: Option<String>,

    /// XLarge image size.
    pub xlarge: Option<String>,

    /// XXLarge image size.
    pub xxlarge: Option<String>,
}

/// Balance that a user carries.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Balance {
    /// Currency code.
    pub currency_code: Option<String>,

    /// Amount of money in the balance.
    pub amount: Option<String>,
}

/// Debt relationship between two users.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
