use axum::{routing::get, Router};

use crate::{handlers, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/hello/{name}", get(handlers::hello))
        .with_state(state)
}
