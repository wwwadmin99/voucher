use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use super::{ensure_product_exists, ensure_user_exists};
#[allow(unused_imports)]
use crate::{
    error::{AppError, ErrorBody},
    models::{ActivateRequest, ActivateResponse, Activation, SetBalanceRequest, VoucherBalance},
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/balances",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "Voucher balance per product", body = [VoucherBalance]),
        (status = 404, description = "User not found", body = ErrorBody)
    )
)]
pub async fn get_balances(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<VoucherBalance>>, AppError> {
    ensure_user_exists(&state.db, user_id).await?;

    let balances = sqlx::query_as::<_, VoucherBalance>(
        r#"
        select p.id as product_id, p.name as product_name, coalesce(vb.quantity, 0) as quantity
        from products p
        left join voucher_balances vb on vb.product_id = p.id and vb.user_id = $1
        order by p.name
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(balances))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/activations",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "Activation history, newest first", body = [Activation]),
        (status = 404, description = "User not found", body = ErrorBody)
    )
)]
pub async fn get_activations(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Activation>>, AppError> {
    ensure_user_exists(&state.db, user_id).await?;

    let activations = sqlx::query_as::<_, Activation>(
        r#"
        select a.id, a.product_id, p.name as product_name, a.activated_at
        from activations a
        join products p on p.id = a.product_id
        where a.user_id = $1
        order by a.activated_at desc
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(activations))
}

#[utoipa::path(
    post,
    path = "/api/users/{user_id}/activations",
    params(("user_id" = Uuid, Path, description = "User id")),
    request_body = ActivateRequest,
    responses(
        (status = 200, description = "Voucher activated", body = ActivateResponse),
        (status = 404, description = "User or product not found", body = ErrorBody),
        (status = 409, description = "No vouchers left for this product", body = ErrorBody)
    )
)]
pub async fn activate(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<ActivateRequest>,
) -> Result<Json<ActivateResponse>, AppError> {
    ensure_user_exists(&state.db, user_id).await?;
    ensure_product_exists(&state.db, body.product_id).await?;

    let mut tx = state.db.begin().await?;

    let current: Option<i32> = sqlx::query_scalar(
        "select quantity from voucher_balances where user_id = $1 and product_id = $2 for update",
    )
    .bind(user_id)
    .bind(body.product_id)
    .fetch_optional(&mut *tx)
    .await?;

    if current.unwrap_or(0) <= 0 {
        return Err(AppError::OutOfVouchers);
    }

    sqlx::query(
        "update voucher_balances set quantity = quantity - 1 where user_id = $1 and product_id = $2",
    )
    .bind(user_id)
    .bind(body.product_id)
    .execute(&mut *tx)
    .await?;

    let activation = sqlx::query_as::<_, Activation>(
        r#"
        with inserted as (
            insert into activations (user_id, product_id)
            values ($1, $2)
            returning id, product_id, activated_at
        )
        select inserted.id, inserted.product_id, p.name as product_name, inserted.activated_at
        from inserted
        join products p on p.id = inserted.product_id
        "#,
    )
    .bind(user_id)
    .bind(body.product_id)
    .fetch_one(&mut *tx)
    .await?;

    let balance = sqlx::query_as::<_, VoucherBalance>(
        r#"
        select p.id as product_id, p.name as product_name, vb.quantity
        from voucher_balances vb
        join products p on p.id = vb.product_id
        where vb.user_id = $1 and vb.product_id = $2
        "#,
    )
    .bind(user_id)
    .bind(body.product_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ActivateResponse { balance, activation }))
}

#[utoipa::path(
    put,
    path = "/api/users/{user_id}/balances/{product_id}",
    params(
        ("user_id" = Uuid, Path, description = "User id"),
        ("product_id" = Uuid, Path, description = "Product id")
    ),
    request_body = SetBalanceRequest,
    responses(
        (status = 200, description = "Balance set", body = VoucherBalance),
        (status = 404, description = "User or product not found", body = ErrorBody),
        (status = 422, description = "Negative quantity", body = ErrorBody)
    )
)]
pub async fn set_balance(
    State(state): State<AppState>,
    Path((user_id, product_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<SetBalanceRequest>,
) -> Result<Json<VoucherBalance>, AppError> {
    if body.quantity < 0 {
        return Err(AppError::InvalidQuantity);
    }

    ensure_user_exists(&state.db, user_id).await?;
    ensure_product_exists(&state.db, product_id).await?;

    sqlx::query(
        r#"
        insert into voucher_balances (user_id, product_id, quantity)
        values ($1, $2, $3)
        on conflict (user_id, product_id)
        do update set quantity = excluded.quantity
        "#,
    )
    .bind(user_id)
    .bind(product_id)
    .bind(body.quantity)
    .execute(&state.db)
    .await?;

    let balance = sqlx::query_as::<_, VoucherBalance>(
        r#"
        select p.id as product_id, p.name as product_name, vb.quantity
        from voucher_balances vb
        join products p on p.id = vb.product_id
        where vb.user_id = $1 and vb.product_id = $2
        "#,
    )
    .bind(user_id)
    .bind(product_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(balance))
}
