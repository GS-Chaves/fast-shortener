use anyhow::Context;
use std::net::SocketAddr;
use tracing::info;

mod handlers;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = state::AppState::new();
    let app = routes::router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}; the port may already be in use"))?;

    axum::serve(listener, app)
        .await
        .context("server terminated unexpectedly")?;

    Ok(())
}
