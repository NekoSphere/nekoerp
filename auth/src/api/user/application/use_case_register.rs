use super::prelude::*;

pub async fn register<R, S>(
    State(usecase): State<Arc<UseCase<R, S>>>,
    Json(payload): Json<UserCreate>,
) -> Result<impl IntoResponse, ApplicationError>
where
    R: RepoBounds,
    S: ServiceBounds,
{
    let UserCreate {
        email,
        password,
        avatar,
        birthday,
        name,
        policies,
    } = payload;

    let email: String = UserCreateEmail::new(&email)?.into();
    let password: String = UserCreatePassword::new(&password)?.into();
    let avatar: String = UserCreateAvatar::new(&avatar)?.into();
    let name: String = UserCreateName::new(&name)?.into();
    let policies: Policies = UserCreatePolicies::new(policies)?.into();
    let birthday: NaiveDate = UserCreateBirthday::new(birthday)?.into();

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

    create_token(&mut user).await?;
    repository.create(&user).await?;

    Ok((StatusCode::CREATED, user.token))
}
