use super::prelude::*;

pub async fn create_token(user: &mut User) -> Result<(), ApplicationError> {
    let uuid = user.uuid.to_string();
    let key = env::var("AES_SECRET_KEY").expect("AES_SECRET_KEY must be provided");
    let mut aes = AesGcmSiv::new()
        .key(AesGcmSivKey::new(key)?)
        .target(AesGcmSivTarget::new(uuid)?);
    let nonce = aes.nonce.clone();
    let token = aes
        .encrypt()
        .map_err(|err| ApplicationError::Internal(format!("Aes encrypt failed {err}")))?;
    let token: String = UserToken::new(format!("{token} {nonce}"))?.into();

    user.mut_token(UserToken::new(&token)?);
    Ok(())
}
