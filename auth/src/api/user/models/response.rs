pub use super::prelude::*;

#[derive(Builder)]
pub struct UserReponse {
    pub uuid: Uuid,
    pub name: String,
    pub birthday: NaiveDate,
    pub email: String,
    pub token: String,
    pub avatar: String,
    pub policies: Policies,
}