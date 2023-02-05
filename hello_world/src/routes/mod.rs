mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_headers;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod middleware_message;

use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_headers::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use middleware_message::middleware_message;

use axum::{
    http::Method,
    routing::{get, post},
    Router, Extension,
};
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData{ message: "Hello from shared data".to_owned() };
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
}
