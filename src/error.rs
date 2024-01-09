#![allow(clippy::enum_variant_names)]

use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub(super) enum ConfigError {
    #[error("failed to access config file: {0}")]
    FailedToLoadConfig(#[from] std::io::Error),
    #[error("failed to deserialize config: {0}")]
    FailedToDeserializeConfig(#[from] toml::de::Error),
    #[error("failed to serialize config: {0}")]
    FailedToSerializeConfig(#[from] toml::ser::Error),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RegisterError {
    #[error("user_name is not alphanumeric")]
    UserNameIsNotAlphanumeric,
    #[error("user_name is too short")]
    UserNameTooShort,
    #[error("user_name is too long")]
    UserNameTooLong,
    #[error("invalid password")]
    InvalidPassword,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("invalid email")]
    InvalidEmail,

    #[error("user_name is taken")]
    UserNameIsTaken,
    #[error("email is taken")]
    EmailIsTaken,

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let error_code = match self {
            Self::UserNameIsNotAlphanumeric | Self::DatabaseError(_) => "-1",
            Self::UserNameTooShort => "-9",
            Self::UserNameTooLong => "-4",
            Self::InvalidPassword | Self::PasswordTooLong => "-5",
            Self::PasswordTooShort => "-8",
            Self::InvalidEmail => "-6",

            Self::UserNameIsTaken => "-2",
            Self::EmailIsTaken => "-3",
        };

        error_code.into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LoginError {
    #[error("incorrect credentials")]
    IncorrectCredentials,

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        let error_code = match self {
            Self::IncorrectCredentials => "-11",
            Self::DatabaseError(_) => "-1",
        };

        error_code.into_response()
    }
}
