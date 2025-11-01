use super::prelude::*;

pub async fn register_first_user<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    Json(payload): Json<UserCreate>,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let user_create = user_create_validation(
        payload.policies(UserCreatePolicies(Policies::first_user_policies())),
    )?;

    let UseCase {
        repository,
        service: _,
    } = &*usecase;

    if repository.count().await? != 0 {
        return Err(ApplicationError::FirstUserExists);
    }

    user_register(Arc::clone(&usecase), user_create).await
}
