pub mod products;
pub mod users;
pub mod vouchers;

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

pub(crate) async fn ensure_user_exists(db: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    let exists: bool = sqlx::query_scalar("select exists(select 1 from users where id = $1)")
        .bind(user_id)
        .fetch_one(db)
        .await?;
    if exists {
        Ok(())
    } else {
        Err(AppError::UserNotFound)
    }
}

pub(crate) async fn ensure_product_exists(db: &PgPool, product_id: Uuid) -> Result<(), AppError> {
    let exists: bool = sqlx::query_scalar("select exists(select 1 from products where id = $1)")
        .bind(product_id)
        .fetch_one(db)
        .await?;
    if exists {
        Ok(())
    } else {
        Err(AppError::ProductNotFound)
    }
}
