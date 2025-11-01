pub mod api;
pub use api::prelude::*;
use axum::{Router, middleware, routing::post};
use dotenv::dotenv;

pub const FORGOT_PASSWORD_PATH: &str = "/forgot";
pub const REGISTER_FIRST_USER_PATH: &str = "/register_first_user";
pub const REGISTER_NEW_USER_PATH: &str = "/register_new_user";
pub const SIGNIN_PATH: &str = "/signin";
pub const TOKEN_PATH: &str = "/token";

pub fn app<R, S>(usecase: Arc<UseCase<R, S>>) -> Router
where
    R: RepoBounds,
    S: ServiceBounds,
{
    dotenv().ok();
    Router::new()
        .route(REGISTER_FIRST_USER_PATH, post(register_first_user::<R, S>))
        .route(REGISTER_NEW_USER_PATH, post(register::<R, S>))
        .route(SIGNIN_PATH, post(signin::<R, S>))
        .route(TOKEN_PATH, post(token::<R, S>))
        .with_state(usecase)
        .layer(middleware::from_fn(log_middleware))
}
