pub use super::prelude::*;

pub async fn user_register<R, S>(
    usecase: Arc<UseCase<R, S>>,
    UserCreate {
        email,
        password,
        name,
        birthday,
        avatar,
        policies,
    }: UserCreate,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let UseCase {
        repository,
        service: _,
    } = &*usecase;
    if let Some(user) = repository.find_by_email(&email).await? {
        user.print().err().await;
        return Err(ApplicationError::UserExists);
    }

    let secret = env::var("ARGON_SECRET_KEY").expect("ARGON_SECRET_KEY must be provided");
    let password = Argon::new()
        .secret(ArgonSecret::new(secret)?)
        .password(ArgonPassword::new(password)?)
        .encrypt()
        .map_err(|err| ApplicationError::Internal(format!("Argon2 encrypt failed {err}")))?;

    let mut user = User::new()
        .email(UserEmail(email))
        .password(UserPassword(password))
        .avatar(UserAvatar(avatar))
        .name(UserName(name))
        .policies(UserPolicies(policies))
        .birthday(UserBirthday(birthday));

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
    let token = user.token.clone();

    repository.create(&user).await?;

    Ok((StatusCode::CREATED, token))
}
