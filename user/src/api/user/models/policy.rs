use std::collections::HashMap;

pub use super::prelude::*;

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize, Copy)]
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

#[derive(Builder, NekoPrint, Debug, Clone, Serialize, Deserialize, Copy)]
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

impl From<Policies> for HashMap<&'static str, Policy> {
    fn from(p: Policies) -> Self {
        HashMap::from([
            ("inventory", p.inventory),
            ("finance", p.finance),
            ("chat", p.chat),
            ("crm", p.crm),
            ("manage_users", p.manage_users),
            ("purchasing", p.purchasing),
            ("kanban", p.kanban),
            ("billing", p.billing),
            ("hr", p.hr),
            ("sales", p.sales),
        ])
    }
}

impl<'a> From<&'a Policies> for HashMap<&'static str, &'a Policy> {
    fn from(p: &'a Policies) -> Self {
        HashMap::from([
            ("inventory", &p.inventory),
            ("finance", &p.finance),
            ("chat", &p.chat),
            ("crm", &p.crm),
            ("manage_users", &p.manage_users),
            ("purchasing", &p.purchasing),
            ("kanban", &p.kanban),
            ("billing", &p.billing),
            ("hr", &p.hr),
            ("sales", &p.sales),
        ])
    }
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
