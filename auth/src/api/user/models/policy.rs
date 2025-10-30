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
    pub invetory: Policy,
    pub finance: Policy,
    pub chat: Policy,
}
