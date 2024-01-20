use axum::{extract::State, response::IntoResponse, Form};
use sqlx::query;

use crate::error::Error;
use crate::{
    error::{GetGJAccountCommentsError, Result},
    forms, utils, AppState,
};

pub(super) async fn get_account_comments(
    State(state): State<AppState>,
    Form(payload): Form<forms::comments::GetGJAccountComments>,
) -> Result<impl IntoResponse> {
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
            utils::timestamp_to_relative(comment.timestamp as u64)
        ));

        if idx < count - 1 {
            response.push('|');
        }
    }

    response.push_str(&format!("#{}:{}:10", count, offset));

    Ok(response)
}

pub(super) async fn upload_account_comment(
    State(state): State<AppState>,
    Form(payload): Form<forms::comments::UploadGJAccComment>,
) -> Result<impl IntoResponse> {
    let result = query!(
        "SELECT `password` FROM `users` WHERE `id` = ?",
        payload.account_id
    )
    .fetch_one(&state.pool)
    .await?;

    if result.password != payload.gjp2 {
        return Err(Error::IncorrectGJP2);
    }

    let chk = utils::generate_chk(
        &[
            &payload.user_name,
            &payload.comment,
            &"0".into(),
            &"0".into(),
            &"1".into(),
        ],
        "xPT6iUrtws0J",
        "29481",
    )
    .expect("should never panic");

    if chk != payload.chk {
        return Err(Error::IncorrectChk);
    }

    let id = query!(
        "INSERT INTO `account_comments` (`text`, `user_id`) VALUES (?, ?) RETURNING `id`",
        payload.comment,
        payload.account_id
    )
    .fetch_one(&state.pool)
    .await?
    .id;

    Ok(format!("{id}"))
}
