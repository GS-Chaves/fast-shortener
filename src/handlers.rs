use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Json<HelloResponse> {
    let key = format!("hello:{name}");
    state.cache.insert(key, name.clone()).await;

    Json(HelloResponse {
        message: format!("Hello, {name}!"),
    })
}
