pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct User {
    #[opt(
        pattern = UUID_PATTERN,
        err = UUID_ERR,
        default = Uuid::new_v4()
    )]
    pub uuid: Uuid,
    pub serial_id: u128,
    #[opt(
        pattern = EMAIL_PATTERN,
        err = EMAIL_ERR,
        default = EMAIL_DEFAULT
    )]
    pub email: String,
    #[opt(
        pattern = PASSWORD_PATTERN,
        err = PASSWORD_ERR,
        default = PASSWORD_DEFAULT
    )]
    pub password: String,
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
    pub modules: Modules,
}
