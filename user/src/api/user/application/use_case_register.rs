use super::prelude::*;

pub async fn register<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    Json(payload): Json<UserCreate>,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let user_create = user_create_validation(payload)?;
    user_register(usecase, user_create).await
}
