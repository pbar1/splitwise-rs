use std::collections::HashMap;

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
    pub errors: Option<HashMap<String, Vec<String>>>,
}

pub(crate) fn join_errors(errors: &HashMap<String, Vec<String>>) -> String {
    let mut error_text = String::from("");
    for (k, v) in errors {
        error_text.push_str(&format!("{}: [{}];", k, v.join("; ")));
    }
    error_text
}
