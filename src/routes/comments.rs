use axum::{extract::State, response::IntoResponse, Form};
use sqlx::query;

use crate::{error::GetGJAccountCommentsError, forms, AppState};

pub(super) async fn get_account_comments(
    State(state): State<AppState>,
    Form(payload): Form<forms::comments::GetGJAccountComments>,
) -> Result<impl IntoResponse, GetGJAccountCommentsError> {
    let offset = payload.page * 10;
    let user_id = payload
        .account_id()
        .ok_or(GetGJAccountCommentsError::NoTargetAccountIDSpecified)?;

    let comments = query!("SELECT * FROM `account_comments` WHERE `user_id` = ? ORDER BY `timestamp` DESC LIMIT 10 OFFSET ?", user_id, offset)
        .fetch_all(&state.pool)
        .await?;

    if comments.is_empty() {
        return Ok("#0:0:0".into());
    }

    let count = comments.len();

    let mut response = String::with_capacity(50);
    for (idx, comment) in comments.into_iter().enumerate() {
        response.push_str(&format!(
            "2~{}~3~{}~4~{}~5~{}~6~{}~7~{}~8~{1}~9~{}",
            comment.text,
            comment.user_id,
            comment.likes,
            comment.dislikes,
            comment.id,
            comment.is_spam,
            comment.timestamp
        ));

        if idx < count - 1 {
            response.push('|');
        }
    }

    response.push_str(&format!("#{}:{}:10", count, offset));

    Ok(response)
}
