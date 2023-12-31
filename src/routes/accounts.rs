use axum::{extract::State, Form};
use sha1_smol::Sha1;
use sqlx::query;

use crate::{error::RegisterError, forms, AppState};

pub(super) async fn register<'a>(
    State(state): State<AppState>,
    Form(payload): Form<forms::accounts::RegisterGJAccount>,
) -> Result<&'static str, RegisterError> {
    if !payload.user_name.chars().all(char::is_alphanumeric) {
        return Err(RegisterError::UserNameIsNotAlphanumeric);
    } else if payload.user_name.len() < 3 {
        return Err(RegisterError::UserNameTooShort);
    } else if payload.user_name.len() > 20 {
        return Err(RegisterError::UserNameTooLong);
    } else if !payload
        .password
        .chars()
        .all(|char| char.is_alphanumeric() || char == '-' || char == '_')
    {
        return Err(RegisterError::InvalidPassword);
    } else if payload.password.len() < 3 {
        return Err(RegisterError::PasswordTooShort);
    } else if payload.password.len() > 20 {
        return Err(RegisterError::PasswordTooLong);
    } else if !email_address::EmailAddress::is_valid(&payload.email) {
        return Err(RegisterError::InvalidEmail);
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `accounts` WHERE `user_name` = ?",
        payload.user_name
    )
    .fetch_one(state.pool())
    .await?
    .count;

    if count > 0 {
        return Err(RegisterError::UserNameIsTaken);
    }

    let count = query!(
        "SELECT COUNT(1) as count FROM `accounts` WHERE `email` = ?",
        payload.email
    )
    .fetch_one(state.pool())
    .await?
    .count;

    if count > 0 {
        return Err(RegisterError::EmailIsTaken);
    }

    let mut sha1 = Sha1::from(payload.password);
    sha1.update(b"mI29fmAnxgTs");
    let hexdigest = sha1.hexdigest();

    query!(
        "INSERT INTO `accounts` (user_name, email, password) VALUES (?, ?, ?)",
        payload.user_name,
        payload.email,
        hexdigest
    )
    .execute(state.pool())
    .await?;

    Ok("1")
}
