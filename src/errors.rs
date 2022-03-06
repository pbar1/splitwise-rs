use serde::{Deserialize, Serialize};

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
pub struct Success {
    pub success: bool,
    pub errors: Option<Vec<String>>,
}
