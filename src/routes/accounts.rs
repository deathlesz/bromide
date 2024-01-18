use axum::{extract::State, response::IntoResponse, Form};
use sqlx::query;

use crate::{
    error::{LoginGJAccountError, RegisterGJAccountError},
    forms, utils, AppState,
};

pub(super) async fn register(
    State(state): State<AppState>,
    Form(payload): Form<forms::accounts::RegisterGJAccount>,
) -> Result<impl IntoResponse, RegisterGJAccountError> {
    if !payload.user_name.chars().all(char::is_alphanumeric) {
        return Err(RegisterGJAccountError::UserNameIsNotAlphanumeric);
    } else if payload.user_name.len() < 3 {
        return Err(RegisterGJAccountError::UserNameTooShort);
    } else if payload.user_name.len() > 20 {
        return Err(RegisterGJAccountError::UserNameTooLong);
    } else if !payload
        .password
        .chars()
        .all(|char| char.is_alphanumeric() || char == '-' || char == '_')
    {
        return Err(RegisterGJAccountError::InvalidPassword);
    } else if payload.password.len() < 3 {
        return Err(RegisterGJAccountError::PasswordTooShort);
    } else if payload.password.len() > 20 {
        return Err(RegisterGJAccountError::PasswordTooLong);
    } else if !email_address::EmailAddress::is_valid(&payload.email) {
        return Err(RegisterGJAccountError::InvalidEmail);
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `users` WHERE `user_name` = ?",
        payload.user_name
    )
    .fetch_one(&state.pool)
    .await?
    .count;

    if count > 0 {
        return Err(RegisterGJAccountError::UserNameIsTaken);
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `users` WHERE `email` = ?",
        payload.email
    )
    .fetch_one(&state.pool)
    .await?
    .count;

    if count > 0 {
        return Err(RegisterGJAccountError::EmailIsTaken);
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
) -> Result<impl IntoResponse, LoginGJAccountError> {
    let result = query!(
        "SELECT `id`, `password` FROM `users` WHERE `user_name` = ?",
        payload.user_name
    )
    .fetch_one(&state.pool)
    .await?;

    if result.password != payload.gjp2 {
        return Err(LoginGJAccountError::IncorrectCredentials);
    }

    Ok(format!("{0},{0}", result.id))
}
