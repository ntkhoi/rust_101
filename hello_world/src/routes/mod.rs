mod hello_world;
use hello_world::hello_world;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
}
