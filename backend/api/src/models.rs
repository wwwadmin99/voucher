use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct VoucherBalance {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct Activation {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ActivateRequest {
    pub product_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ActivateResponse {
    pub balance: VoucherBalance,
    pub activation: Activation,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetBalanceRequest {
    pub quantity: i32,
}
