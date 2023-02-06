pub mod routes;

use routes::create_routes;
use sea_orm::Database;

pub async fn run_database(database_uri: &str) {
    let database = Database::connect(database_uri).await
}

pub async fn run() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = create_routes();
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
