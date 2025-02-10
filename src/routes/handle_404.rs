use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, r#"what, this ain't right chief"#)
}
