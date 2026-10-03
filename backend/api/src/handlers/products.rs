use axum::{extract::State, Json};

use crate::{error::AppError, models::Product, state::AppState};

#[utoipa::path(
    get,
    path = "/api/products",
    responses((status = 200, description = "List all products", body = [Product]))
)]
pub async fn list_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, AppError> {
    let products = sqlx::query_as::<_, Product>("select id, name from products order by name")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(products))
}
