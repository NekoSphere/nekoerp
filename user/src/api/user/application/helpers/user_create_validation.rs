pub use super::prelude::*;

pub fn user_create_validation(payload: UserCreate) -> Result<UserCreate, ApplicationError> {
    let UserCreate {
        email,
        password,
        avatar,
        birthday,
        name,
        policies,
    } = &payload;

    UserCreateEmail::new(&email)?;
    UserCreatePassword::new(&password)?;
    UserCreateAvatar::new(&avatar)?;
    UserCreateName::new(&name)?;
    UserCreatePolicies::new(*policies)?;
    UserCreateBirthday::new(*birthday)?;

    Ok(payload)
}
