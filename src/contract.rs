#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:execute-buy-storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        BuyStorage {
            for_address,
            duration,
            bytes,
            payment_denom,
        } => execute::buy_storage(deps, env, info, for_address, duration, bytes, payment_denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

mod execute {
    use crate::error::ContractError;
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

    pub fn buy_storage(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        for_address: String,
        duration: i64,
        bytes: i64,
        payment_denom: String,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
}
#[cfg(test)]
mod tests {}
