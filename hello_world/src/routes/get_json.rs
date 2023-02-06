use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data{ message: "Todo".to_owned(), count:10, username: "ntk".to_owned() };
    Json(data)
}