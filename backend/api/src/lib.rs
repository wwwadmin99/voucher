pub mod error;
pub mod handlers;
pub mod models;
pub mod openapi;
pub mod state;

use axum::{
    routing::{get, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{openapi::ApiDoc, state::AppState};

pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub fn app_router(state: AppState, static_dir: Option<&str>) -> Router {
    let api_routes = Router::new()
        .route("/users", get(handlers::users::list_users))
        .route("/products", get(handlers::products::list_products))
        .route(
            "/users/:user_id/balances",
            get(handlers::vouchers::get_balances),
        )
        .route(
            "/users/:user_id/balances/:product_id",
            put(handlers::vouchers::set_balance),
        )
        .route(
            "/users/:user_id/activations",
            get(handlers::vouchers::get_activations).post(handlers::vouchers::activate),
        )
        .with_state(state);

    let router = Router::new()
        .nest("/api", api_routes)
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()));

    let router = if let Some(static_dir) = static_dir {
        let index_html = format!("{static_dir}/index.html");
        let serve_frontend = ServeDir::new(static_dir).fallback(ServeFile::new(index_html));
        router.fallback_service(serve_frontend)
    } else {
        router
    };

    router
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
