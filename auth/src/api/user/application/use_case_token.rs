use super::prelude::*;

pub async fn token<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    Json(token): Json<String>,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let [token, nonce]: [&str; 2] = token
        .split(' ')
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .map_err(|err| ApplicationError::Validation(format!("Token must be two parts {err}")))?;
    let UseCase {
        repository,
        service: _,
    } = &*usecase;

    let key = env::var("AES_SECRET_KEY").expect("AES_SECRET_KEY must be provided");

    let uuid = AesGcmSiv::new()
        .target(AesGcmSivTarget::new(token)?)
        .key(AesGcmSivKey::new(key)?)
        .nonce(AesGcmSivNonce::new(nonce)?)
        .decrypt()
        .map_err(|err| ApplicationError::Internal(format!("Aes decrypt failed {err}")))?
        .to_string();

    if let Some(user) = repository.find_by_id(uuid).await? {
        return Ok((StatusCode::OK, Json(user)));
    }

    Err(ApplicationError::UserNotFound)
}
