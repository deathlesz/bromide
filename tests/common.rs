use axum::{
    body::Body,
    http::{request, Request},
};
use serde::Serialize;

pub trait RequestBuilderExt {
    fn form(self, form: &(impl Serialize + ?Sized)) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
    fn form(self, form: &(impl Serialize + ?Sized)) -> Request<Body> {
        let body_text = serde_urlencoded::to_string(form)
            .expect("failed to serialize into urlencoded form data");

        self.header("Content-Type", "application/x-www-form-urlencoded")
            .body(body_text.into())
            .expect("failed to build request")
    }
}

pub async fn body_into_string(body: Body) -> String {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    std::str::from_utf8(&bytes)
        .expect("body should be in utf-8")
        .to_string()
}
