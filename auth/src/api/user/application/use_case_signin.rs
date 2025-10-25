use super::prelude::*;

pub async fn signin<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    Json(payload): Json<UserAuth>,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let UserAuth { email, password } = payload;
    let email: String = UserAuthEmail::new(&email)?.into();
    let password: String = UserAuthPassword::new(&password)?.into();
    let UseCase {
        cache,
        repository,
        service: _,
    } = &*usecase;
    cache.find_by_email(&email).await;
    repository.find_by_email(email).await;

    Ok(())
}
