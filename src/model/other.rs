use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{expenses::Expense, Image};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CurrenciesWrapper {
    pub currencies: Vec<Currency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CategoriesWrapper {
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub icon_types: Option<HashMap<String, Image>>,
    pub subcategories: Option<Vec<Category>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceRequest {
    pub input: String,
    pub friend_id: Option<i64>,
    pub group_id: Option<i64>,
    pub autosave: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceResponse {
    pub expense: Option<Expense>,
    pub valid: Option<bool>,
    pub confidence: Option<f64>,
    pub error: Option<String>,
}
