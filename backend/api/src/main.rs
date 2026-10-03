use std::net::SocketAddr;

use voucher_api::{app_router, connect, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;
    let db = connect(&database_url).await?;
    let state = AppState { db };

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "static".to_string());
    let app = app_router(state, Some(&static_dir));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
