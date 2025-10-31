use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Modules {
    pub inventory: bool,
    pub finance: bool,
    pub chat: bool,
    pub crm: bool,
    pub purchasing: bool,
    pub kanban: bool,
    pub billing: bool,
    pub hr: bool,
    pub sales: bool,
}
