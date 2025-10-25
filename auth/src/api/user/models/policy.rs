pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Policy {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
}

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Policies {
    pub crm: Policy,
    pub inventory: Policy,
    pub billing: Policy,
    pub chat: Policy,
    pub kanban: Policy,
    pub finance: Policy,
    pub hr: Policy,
    pub purchasing: Policy,
    pub sales: Policy,
    pub manage_users: Policy,
}
