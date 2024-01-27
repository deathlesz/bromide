#![allow(clippy::enum_variant_names)]

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(super) enum ConfigError {
    #[error("failed to access config file: {0}")]
    FailedToLoadConfig(#[from] std::io::Error),
    #[error("failed to deserialize config: {0}")]
    FailedToDeserializeConfig(#[from] toml::de::Error),
    #[error("failed to serialize config: {0}")]
    FailedToSerializeConfig(#[from] toml::ser::Error),
}

#[derive(Debug, thiserror::Error, response_error::ResponseError)]
pub(crate) enum Error {
    #[error("generic error occured")]
    Generic,

    #[error("incorrect gjp2")]
    IncorrectGJP2,
    #[error("incorrect chk/seed2")]
    IncorrectChk,
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    #[response(transparent)]
    RegisterGJAccountError(#[from] RegisterGJAccountError),
    #[error(transparent)]
    #[response(transparent)]
    LoginGJAccountError(#[from] LoginGJAccountError),
    #[error(transparent)]
    #[response(transparent)]
    GetGJAccountCommentsError(#[from] GetGJAccountCommentsError),
}

#[derive(Debug, thiserror::Error, response_error::ResponseError)]
pub(crate) enum RegisterGJAccountError {
    #[error("user_name is not alphanumeric")]
    UserNameIsNotAlphanumeric,
    #[error("user_name is too short")]
    #[response(error_code = "-9")]
    UserNameTooShort,
    #[error("user_name is too long")]
    #[response(error_code = "-4")]
    UserNameTooLong,
    #[error("invalid password")]
    #[response(error_code = "-5")]
    InvalidPassword,
    #[error("password is too short")]
    #[response(error_code = "-8")]
    PasswordTooShort,
    #[error("password is too long")]
    #[response(error_code = "-5")]
    PasswordTooLong,
    #[error("invalid email")]
    #[response(error_code = "-6")]
    InvalidEmail,

    #[error("user_name is taken")]
    #[response(error_code = "-2")]
    UserNameIsTaken,
    #[error("email is taken")]
    #[response(error_code = "-3")]
    EmailIsTaken,
}

#[derive(Debug, thiserror::Error, response_error::ResponseError)]
pub(crate) enum LoginGJAccountError {
    #[error("incorrect credentials")]
    #[response(error_code = "-11")]
    IncorrectCredentials,
}

#[derive(Debug, thiserror::Error, response_error::ResponseError)]
pub(crate) enum GetGJAccountCommentsError {
    #[error("no target account ID specified")]
    NoTargetAccountIDSpecified,
}
