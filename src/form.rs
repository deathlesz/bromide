use axum::{
    extract::{FromRequest, RawForm, Request},
    RequestExt as _,
};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Form<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        if let Ok(RawForm(bytes)) = req.extract().await {
            let value = serde_urlencoded::from_bytes(&bytes).map_err(|_| "-1")?;

            Ok(Form(value))
        } else {
            Err("-1")
        }
    }
}

impl<T> std::ops::Deref for Form<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Form<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
