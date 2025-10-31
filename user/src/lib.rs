pub mod api;
pub use api::prelude::*;
use axum::{Router, routing::post};
use dotenv::dotenv;

pub fn app<R, S>(usecase: Arc<UseCase<R, S>>) -> Router
where
    R: RepoBounds,
    S: ServiceBounds,
{
    dotenv().ok();
    Router::new()
        .route("/register", post(register::<R, S>))
        .route("/signin", post(signin::<R, S>))
        .route("/token", post(token::<R, S>))
        .with_state(usecase)
}
