use hello_world::{run,run_database};
use dotenvy_macro::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run_database(database_uri).await;
    run().await
}
