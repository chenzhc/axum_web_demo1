use axum::{debug_handler, Json};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Vehice {
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

pub async fn vehice_post() {

}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehice> {
    println!("Caller retrieved a vehice from axum");
    Json::from(Vehice {
        manufacturer: String::from("Dog"),
        model: "RAM 1500".to_string(),
        year: 2025,
        id: uuid::Uuid::new_v4().to_string(),
    })
}