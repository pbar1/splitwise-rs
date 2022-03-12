use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::expenses::Expense;
use crate::model::shared::Image;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CurrenciesWrapper {
    pub currencies: Vec<Currency>,
}

/// Currency supported by Splitwise.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    /// Mostly ISO 4217 currency codes, along with colloquial codes such as
    /// "BTC".
    pub currency_code: Option<String>,

    /// Currency unit symbol.
    pub unit: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CategoriesWrapper {
    pub categories: Vec<Category>,
}

/// Category for expenses.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    /// Category ID.
    pub id: Option<i64>,

    /// Category name.
    pub name: Option<String>,

    /// Category icon.
    pub icon: Option<String>,

    /// Category icon types.
    pub icon_types: Option<HashMap<String, Image>>,

    /// Subcategories.
    pub subcategories: Option<Vec<Category>>,
}

/// Splitwise `parse_sentence` request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceRequest {
    /// A natural language sentence describing an expense.
    pub input: String,

    /// User ID.
    pub friend_id: Option<i64>,

    /// Group ID.
    pub group_id: Option<i64>,

    /// If true, the resulting expense will be saved if valid.
    pub autosave: bool,
}

/// Splitwise `parse_sentence` response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceResponse {
    /// Expense object parsed from the sentence input.
    pub expense: Option<Expense>,

    /// Whether the parsed input was a valid expense.
    pub valid: Option<bool>,

    /// Confidence value for the parsed expense.
    pub confidence: Option<f64>,

    /// Error that occurred during this request.
    pub error: Option<String>,
}
