use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub payment: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    BuyStorage {
        for_address: String,
        duration: i64,
        bytes: i64,
        payment_denom: String,
    },

    MsgBuyStorage {
        creator: String,
        for_address: String,
        duration_days: i64,
        bytes: i64,
        payment_denom: String,
        referral: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
