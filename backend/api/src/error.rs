use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    UserNotFound,
    ProductNotFound,
    OutOfVouchers,
    InvalidQuantity,
    Internal(anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Internal(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error, message) = match self {
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "user_not_found",
                "Пользователь не найден".to_string(),
            ),
            AppError::ProductNotFound => (
                StatusCode::NOT_FOUND,
                "product_not_found",
                "Продукт не найден".to_string(),
            ),
            AppError::OutOfVouchers => (
                StatusCode::CONFLICT,
                "out_of_vouchers",
                "Ваучеров на этот продукт не осталось".to_string(),
            ),
            AppError::InvalidQuantity => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_quantity",
                "Количество ваучеров не может быть отрицательным".to_string(),
            ),
            AppError::Internal(err) => {
                tracing::error!(?err, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "Внутренняя ошибка сервера".to_string(),
                )
            }
        };

        (
            status,
            Json(ErrorBody {
                error: error.to_string(),
                message,
            }),
        )
            .into_response()
    }
}
