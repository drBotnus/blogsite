use std::io;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() -> io::Result<()> {
    let router = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("->> axum listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router).await
}

async fn index() -> impl IntoResponse {
    (StatusCode::OK, "Homepage")
}
