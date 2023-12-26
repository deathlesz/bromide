use axum::http::{StatusCode, Uri};

pub(super) async fn unhandled(uri: Uri, body: String) -> StatusCode {
    println!("Got unhandled request:");
    println!("\tURI: {:?}", uri);

    println!("\tParams:");
    body.split('&')
        .for_each(|pair| println!("\t\t{pair}"));

    StatusCode::IM_A_TEAPOT
}
