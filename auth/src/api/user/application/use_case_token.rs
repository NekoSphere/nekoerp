use super::prelude::*;

pub async fn token<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let auth = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ApplicationError::Validation("Authorization header missing".into()))?;
    let auth = auth.to_str().map_err(|_| {
        ApplicationError::Validation("Authorization header is not valid UTF-8".into())
    })?;

    let [token, nonce]: [&str; 2] = auth
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
        .ciphertext(AesGcmSivCiphertext::new(token)?)
        .decrypt()
        .map_err(|err| ApplicationError::Internal(format!("Aes decrypt failed {err}")))?
        .to_string();

    if let Some(user) = repository.find_by_id(uuid).await? {
        return Ok((StatusCode::OK, Json(user)));
    }

    Err(ApplicationError::UserNotFound)
}
