use axum::{extract::State, response::IntoResponse, Form};
use sqlx::query;

use crate::{
    error::{LoginGJAccountError, RegisterGJAccountError, Result},
    forms, utils, AppState,
};

pub(super) async fn register(
    State(state): State<AppState>,
    Form(payload): Form<forms::accounts::RegisterGJAccount>,
) -> Result<impl IntoResponse> {
    if !payload.user_name.chars().all(char::is_alphanumeric) {
        return Err(RegisterGJAccountError::UserNameIsNotAlphanumeric.into());
    } else if payload.user_name.len() < 3 {
        return Err(RegisterGJAccountError::UserNameTooShort.into());
    } else if payload.user_name.len() > 20 {
        return Err(RegisterGJAccountError::UserNameTooLong.into());
    } else if !payload
        .password
        .chars()
        .all(|char| char.is_alphanumeric() || char == '-' || char == '_')
    {
        return Err(RegisterGJAccountError::InvalidPassword.into());
    } else if payload.password.len() < 3 {
        return Err(RegisterGJAccountError::PasswordTooShort.into());
    } else if payload.password.len() > 20 {
        return Err(RegisterGJAccountError::PasswordTooLong.into());
    } else if !email_address::EmailAddress::is_valid(&payload.email) {
        return Err(RegisterGJAccountError::InvalidEmail.into());
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `users` WHERE `user_name` = ?",
        payload.user_name
    )
    .fetch_one(&state.pool)
    .await?
    .count;

    if count > 0 {
        return Err(RegisterGJAccountError::UserNameIsTaken.into());
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `users` WHERE `email` = ?",
        payload.email
    )
    .fetch_one(&state.pool)
    .await?
    .count;

    if count > 0 {
        return Err(RegisterGJAccountError::EmailIsTaken.into());
    }

    let hash = utils::password_hash(&payload.password);

    query!(
        "INSERT INTO `users` (`user_name`, `email`, `password`) VALUES (?, ?, ?)",
        payload.user_name,
        payload.email,
        hash
    )
    .execute(&state.pool)
    .await?;

    Ok("1")
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Form(payload): Form<forms::accounts::LoginGJAccount>,
) -> Result<impl IntoResponse> {
    let result = query!(
        "SELECT `id`, `password` FROM `users` WHERE `user_name` = ?",
        payload.user_name
    )
    .fetch_one(&state.pool)
    .await?;

    if result.password != payload.gjp2 {
        return Err(LoginGJAccountError::IncorrectCredentials.into());
    }

    Ok(format!("{0},{0}", result.id))
}
