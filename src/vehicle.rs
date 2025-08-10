use axum::{debug_handler, extract::Query, Json};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Vehice {
    #[serde(default,rename = "manufacturer")]
    manufacturer: String,
    #[serde(default,rename = "model")]
    model: String,
    #[serde(default,rename = "year")]
    year: u32,
    #[serde(default,rename = "id")]
    id: Option<String>,
}

pub async fn vehice_post_query(Query(mut v): Query<Vehice>) -> Json<Vehice> {
    v.id = Some(uuid::Uuid::new_v4().to_string());

    Json::from(v)
}

pub async fn vehice_post(Json(mut v): Json<Vehice>) -> Json<Vehice> {
    println!("manufacturer: {0}, mode:{1}, year: {2}",v.manufacturer,v.model,v.year);
    v.id = Some(uuid::Uuid::new_v4().to_string());

    Json::from(v)
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehice> {
    println!("Caller retrieved a vehice from axum");
    Json::from(Vehice {
        manufacturer: String::from("Dog"),
        model: "RAM 1500".to_string(),
        year: 2025,
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}