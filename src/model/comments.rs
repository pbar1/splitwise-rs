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

/// Comment on an expense.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    /// Comment ID.
    pub id: i64,

    /// Comment contents.
    pub content: String,

    /// Comment type. One of:
    /// - `System`
    /// - `User`
    pub comment_type: String,

    /// Relation type. One of:
    /// - `ExpenseComment`
    pub relation_type: String,

    /// ID of the subject of the comment.
    pub relation_id: i64,

    /// Timestamp of when the comment was created.
    pub created_at: chrono::DateTime<Utc>,

    /// Timestamp of when the comment was deleted.
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    /// User that left the comment.
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreateCommentRequest {
    pub expense_id: i64,
    pub content: String,
}
