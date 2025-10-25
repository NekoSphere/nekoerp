pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct UserAuth {
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
}
