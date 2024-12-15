use axum::{http::StatusCode, response::IntoResponse};

pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Oops! Looks like you took a wrong turn... 404",
    )
        .into_response()
}

pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        "All systems go! Everything is running smooth like butter 🧈",
    )
        .into_response()
}
