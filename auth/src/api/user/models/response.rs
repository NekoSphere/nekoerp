pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone)]
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
    pub token: String,
    pub policies: Policies,
}
