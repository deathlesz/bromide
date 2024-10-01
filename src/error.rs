use sqlx::error::DatabaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generic error occured: {0}")]
    Generic(&'static str),
    #[error("database error occured")]
    Sqlx(#[from] sqlx::Error),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Generic(value)
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Generic(s) => s,
            Self::Sqlx(_) => "-1",
        }
        .into_response()
    }
}

pub trait ResultExt<T, E> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> E,
    ) -> std::result::Result<T, Error>;
}

impl<T, E, R> ResultExt<T, R> for std::result::Result<T, E>
where
    E: Into<Error>,
    R: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> R,
    ) -> std::result::Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe).into()
            }
            e => e,
        })
    }
}
