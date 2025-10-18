pub use super::prelude::*;

#[derive(Builder, Debug)]
pub struct Policy {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
}

#[derive(Builder, Debug)]
pub struct Policies {
    pub crm: Policy,
    pub inventory: Policy,
    pub billing: Policy,
    pub collab: Policy,
    pub finance: Policy,
    pub hr: Policy,
    pub purchasing: Policy,
    pub sales: Policy,
    pub manage_users: Policy
}