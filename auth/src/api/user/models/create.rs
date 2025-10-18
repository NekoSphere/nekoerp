pub use super::prelude::*;

#[derive(Builder)]
pub struct UserCreate {
    pub email: String,
    pub password: String,
    pub name: String,
    pub birthday: NaiveDate,
    pub avatar: String,
    pub policies: Policies,
}