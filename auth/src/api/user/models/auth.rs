pub use super::prelude::*;

#[derive(Builder)]
pub struct UserAuth {
    pub email: String,
    pub password: String
}
