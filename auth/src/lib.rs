pub mod api;
use crate::api::user::application::use_case_register::register;
use api::prelude::*;
use axum::{Router, routing::post};

pub fn app<R, S>(usecase: Arc<UseCase<R, S>>) -> Router
where
    R: RepoBounds,
    S: ServiceBounds,
{
    Router::new()
        .route("/register", post(register::<R, S>))
        .with_state(usecase)
}
