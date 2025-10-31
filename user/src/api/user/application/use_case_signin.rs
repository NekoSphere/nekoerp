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
        repository,
        service: _,
    } = &*usecase;

    if let Some(user) = repository.find_by_email(&email).await? {
        user.print().success().await;
        let hash = user.password;
        let secret = env::var("ARGON_SECRET_KEY").expect("ARGON_SECRET_KEY must be provided");
        Argon::new()
            .secret(ArgonSecret::new(secret)?)
            .password(ArgonPassword::new(password)?)
            .hash(ArgonHash::new(hash)?)
            .verify()
            .map_err(|err| ApplicationError::Internal(format!("Argon2 verify failed {err}")))?;

        return Ok((StatusCode::OK, user.token));
    }

    Err(ApplicationError::WrongEmailOrPassword)
}
