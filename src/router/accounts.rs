use axum::{extract::State, response::IntoResponse, routing::post, Form, Router};

use crate::{
    error::{Result, ResultExt as _},
    schema, utils,
};

async fn register(
    State(pool): State<sqlx::PgPool>,
    Form(data): Form<schema::RegisterGJAccount>,
) -> Result<impl IntoResponse> {
    if data.secret != "Wmfv3899gc9" || !data.user_name.chars().all(char::is_alphanumeric) {
        return Err("-1")?;
    } else if data.user_name.len() < 3 {
        return Err("-9")?;
    } else if data.user_name.len() > 20 {
        return Err("-4")?;
    } else if !data
        .password
        .chars()
        .all(|char| char.is_alphanumeric() || char == '-' || char == '_')
    {
        return Err("-5")?;
    } else if data.password.len() < 3 {
        return Err("-8")?;
    } else if data.password.len() > 20 {
        return Err("-5")?;
    } else if !email_address::EmailAddress::is_valid(&data.email) {
        return Err("-6")?;
    }

    sqlx::query!(
        "insert into accounts (user_name, password, email) values ($1, $2, $3)",
        data.user_name,
        utils::password_hash(data.password),
        data.email
    )
    .execute(&pool)
    .await
    .on_constraint("accounts_user_name_key", |_| "-2")
    .on_constraint("accounts_email_key", |_| "-3")?;

    Ok("1")
}

async fn login(
    State(pool): State<sqlx::PgPool>,
    Form(data): Form<schema::LoginGJAccount>,
) -> Result<impl IntoResponse> {
    if data.secret != "Wmfv3899gc9" {
        return Err("-1")?;
    } else if data.user_name.len() < 6 {
        return Err("-9")?;
    }

    let result = sqlx::query!(
        "select id, password, uid from accounts where user_name = $1",
        data.user_name,
    )
    .fetch_one(&pool)
    .await?;

    if utils::check_password(&result.password, &data.gjp2) {
        Err("-11")?
    } else {
        Ok(format!("{},{}", result.id, result.uid))
    }
}

pub(super) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/accounts/registerGJAccount.php", post(register))
        .route("/accounts/loginGJAccount.php", post(login))
        .with_state(pool)
}
