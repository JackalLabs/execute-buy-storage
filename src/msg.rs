use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    BuyStorage {
        for_address: String,
        duration: i64,
        bytes: i64,
        payment_denom: String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
