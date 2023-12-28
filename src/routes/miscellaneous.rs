use axum::http::{StatusCode, Uri};
use tracing::debug;

pub(super) async fn unhandled(uri: Uri, body: String) -> StatusCode {
    debug!("Got unhandled request:");
    debug!("\tURI: {:?}", uri);

    debug!("\tParams:");
    body.split('&').for_each(|pair| debug!("\t\t{pair}"));

    StatusCode::IM_A_TEAPOT
}
