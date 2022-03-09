use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::model::users::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommentsWrapper {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommentWrapper {
    pub comment: Comment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub comment_type: String,
    pub relation_type: String,
    pub relation_id: i64,
    pub created_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
    pub user: Option<User>, // TODO: Guessing this is the main "User" type
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreateCommentRequest {
    pub expense_id: i64,
    pub content: String,
}
