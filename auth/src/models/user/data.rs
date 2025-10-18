pub use super::prelude::*;

#[derive(Builder)]
pub struct User {
    pub uuid: Uuid,
    pub serial_id: u128,
    pub policies: Policies,
    pub name: String,
    pub birthday: NaiveDate,
    pub email: String,
    pub password: String,
    pub avatar: String,
}

