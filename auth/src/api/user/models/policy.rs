pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Policy {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
}

impl Policy {
    pub fn all() -> Self {
        Self::new()
            .delete(PolicyDelete(true))
            .read(PolicyRead(true))
            .write(PolicyWrite(true))
    }
}

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Policies {
    pub inventory: Policy,
    pub finance: Policy,
    pub chat: Policy,
    pub crm: Policy,
    pub manage_users: Policy,
    pub purchasing: Policy,
    pub kanban: Policy,
    pub billing: Policy,
    pub hr: Policy,
    pub sales: Policy,
}

impl Policies {
    pub fn first_user_policies() -> Self {
        Self::new()
            .chat(PoliciesChat(Policy::all()))
            .finance(PoliciesFinance(Policy::all()))
            .inventory(PoliciesInventory(Policy::all()))
            .crm(PoliciesCrm(Policy::all()))
            .manage_users(PoliciesManageUsers(Policy::all()))
            .purchasing(PoliciesPurchasing(Policy::all()))
            .kanban(PoliciesKanban(Policy::all()))
            .billing(PoliciesBilling(Policy::all()))
            .hr(PoliciesHr(Policy::all()))
            .sales(PoliciesSales(Policy::all()))
    }
}
