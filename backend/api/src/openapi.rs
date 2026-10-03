use utoipa::OpenApi;

use crate::{error::ErrorBody, handlers, models};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::users::list_users,
        handlers::products::list_products,
        handlers::vouchers::get_balances,
        handlers::vouchers::get_activations,
        handlers::vouchers::activate,
        handlers::vouchers::set_balance,
    ),
    components(schemas(
        models::User,
        models::Product,
        models::VoucherBalance,
        models::Activation,
        models::ActivateRequest,
        models::ActivateResponse,
        models::SetBalanceRequest,
        ErrorBody,
    )),
    tags((name = "voucher", description = "Users, products and voucher activations"))
)]
pub struct ApiDoc;
