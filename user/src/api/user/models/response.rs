pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct UserReponse {
    pub uuid: Uuid,
    #[opt(
        pattern = EMAIL_PATTERN,
        err = EMAIL_ERR,
        default = EMAIL_DEFAULT
    )]
    pub email: String,
    #[opt(
        pattern = NAME_PATTERN,
        err = NAME_ERR,
        default = NAME_DEFAULT
    )]
    pub name: String,
    pub birthday: NaiveDate,
    #[opt(
        pattern = AVATAR_PATTERN,
        err = AVATAR_ERR,
        default = AVATAR_DEFAULT
    )]
    pub avatar: String,
    pub policies: Policies,
    pub modules: Modules,
}

impl From<User> for UserReponse {
    fn from(u: User) -> Self {
        UserReponse {
            uuid: u.uuid,
            email: u.email,
            name: u.name,
            birthday: u.birthday,
            avatar: u.avatar,
            policies: u.policies,
            modules: u.modules,
        }
    }
}

impl From<&User> for UserReponse {
    fn from(u: &User) -> Self {
        UserReponse {
            uuid: u.uuid,
            email: u.email.clone(),
            name: u.name.clone(),
            birthday: u.birthday,
            avatar: u.avatar.clone(),
            policies: u.policies.clone(),
            modules: u.modules.clone(),
        }
    }
}
