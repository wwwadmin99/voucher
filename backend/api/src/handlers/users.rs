use axum::{extract::State, Json};

use crate::{error::AppError, models::User, state::AppState};

#[utoipa::path(
    get,
    path = "/api/users",
    responses((status = 200, description = "List all users", body = [User]))
)]
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("select id, name from users order by name")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(users))
}
