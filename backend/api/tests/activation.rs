use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;
use voucher_api::{app_router, state::AppState};

async fn send(app: Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(match body {
            Some(b) => Body::from(serde_json::to_vec(&b).unwrap()),
            None => Body::empty(),
        })
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, json)
}

async fn first_user_id(pool: &PgPool) -> Uuid {
    sqlx::query_scalar("select id from users order by name limit 1")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn first_product_id(pool: &PgPool) -> Uuid {
    sqlx::query_scalar("select id from products order by name limit 1")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn activation_decrements_balance_and_records_history(pool: PgPool) {
    let user_id = first_user_id(&pool).await;
    let product_id = first_product_id(&pool).await;
    let app = app_router(AppState { db: pool }, None);

    let (status, _) = send(
        app.clone(),
        Method::PUT,
        &format!("/api/users/{user_id}/balances/{product_id}"),
        Some(json!({ "quantity": 1 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = send(
        app.clone(),
        Method::POST,
        &format!("/api/users/{user_id}/activations"),
        Some(json!({ "product_id": product_id })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["balance"]["quantity"], 0);

    let (status, body) = send(
        app.clone(),
        Method::GET,
        &format!("/api/users/{user_id}/activations"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[sqlx::test]
async fn activation_fails_when_out_of_vouchers(pool: PgPool) {
    let user_id = first_user_id(&pool).await;
    let product_id = first_product_id(&pool).await;
    let app = app_router(AppState { db: pool }, None);

    let (status, body) = send(
        app.clone(),
        Method::POST,
        &format!("/api/users/{user_id}/activations"),
        Some(json!({ "product_id": product_id })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"], "out_of_vouchers");

    let (status, body) = send(
        app.clone(),
        Method::GET,
        &format!("/api/users/{user_id}/activations"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[sqlx::test]
async fn admin_set_balance_upserts(pool: PgPool) {
    let user_id = first_user_id(&pool).await;
    let product_id = first_product_id(&pool).await;
    let app = app_router(AppState { db: pool }, None);

    let (status, body) = send(
        app.clone(),
        Method::PUT,
        &format!("/api/users/{user_id}/balances/{product_id}"),
        Some(json!({ "quantity": 5 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["quantity"], 5);

    let (status, body) = send(
        app.clone(),
        Method::PUT,
        &format!("/api/users/{user_id}/balances/{product_id}"),
        Some(json!({ "quantity": 3 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["quantity"], 3);
}

#[sqlx::test]
async fn negative_quantity_rejected(pool: PgPool) {
    let user_id = first_user_id(&pool).await;
    let product_id = first_product_id(&pool).await;
    let app = app_router(AppState { db: pool }, None);

    let (status, body) = send(
        app.clone(),
        Method::PUT,
        &format!("/api/users/{user_id}/balances/{product_id}"),
        Some(json!({ "quantity": -1 })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"], "invalid_quantity");
}
