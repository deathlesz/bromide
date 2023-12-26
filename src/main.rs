mod routes;

#[tokio::main]
async fn main() {
    let address = format!("{}:{}",
                          std::env::var("BIND_IP").unwrap_or("0.0.0.0".into()),
                          std::env::var("BIND_PORT").unwrap_or("80".into()));

    axum::serve(
        tokio::net::TcpListener::bind(address).await.expect("failed to bind to address"),
        routes::router()
    ).await.expect("failed to start a server");
}
