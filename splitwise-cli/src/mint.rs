use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintTransaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub id: String,
    pub account_id: String,
    pub account_ref: AccountRef,
    pub date: NaiveDate,
    pub description: String,
    pub category: Category,
    pub amount: f64,
    pub status: String,
    pub match_state: String,
    pub fi_data: FiData,
    pub is_reviewed: bool,
    pub merchant_id: Option<i64>,
    pub etag: String,
    pub is_expense: bool,
    pub is_pending: bool,
    pub discretionary_type: String,
    pub is_linked_to_rule: bool,
    pub transaction_review_state: String,
    pub last_updated_date: DateTime<Utc>,
    pub principal: Option<f64>,
    pub principal_currency: Option<String>,
    pub interest: Option<f64>,
    pub interest_currency: Option<String>,
    pub escrow: Option<f64>,
    pub escrow_currency: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRef {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub hidden_from_planning_and_trends: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub category_type: String,
    pub parent_id: String,
    pub parent_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiData {
    pub id: Option<String>,
    pub date: NaiveDate,
    pub amount: f64,
    pub description: String,
    pub inferred_description: String,
    pub inferred_category: InferredCategory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferredCategory {
    pub id: String,
    pub name: String,
}
