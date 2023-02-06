use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn return_201() -> Response {
    (StatusCode::CREATED, "This is 201").into_response()
}
