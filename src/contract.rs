#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::PAYMENT;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:execute-buy-storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PAYMENT.save(deps.storage, &msg.payment)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ExecuteMsg>, ContractError> {
    use ExecuteMsg::*;

    match msg {
        BuyStorage {
            for_address,
            duration,
            bytes,
            payment_denom,
        } => execute::buy_storage(deps, env, info, for_address, duration, bytes, payment_denom),
        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

mod execute {
    use crate::msg::ExecuteMsg::{self, MsgBuyStorage};
    use crate::{error::ContractError, state::PAYMENT};
    use cosmwasm_std::{to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response};

    pub fn buy_storage(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        for_address: String,
        duration: i64,
        bytes: i64,
        payment_denom: String,
    ) -> Result<Response<ExecuteMsg>, ContractError> {
        let payment: Vec<String> = PAYMENT.load(deps.storage)?;
        if !payment.contains(&payment_denom) {
            return Err(ContractError::InvalidPayment {});
        }

        /*        
        example from canined
        // {"body":{"messages":[{"@type":"/canine_chain.storage.MsgBuyStorage","creator":"jkl1jnysschmhmq0frqr3tung9gvg0rwadjzh82sta","for_address":"jkl10k05lmc88q5ft3lm00q30qkd9x6654h3lejnct","duration_days":"31","bytes":"1099511621","payment_denom":"ujkl","referral":""}],"memo":"","timeout_height":"0","extension_options":[],"non_critical_extension_options":[]},"auth_info":{"signer_infos":[],"fee":{"amount":[{"denom":"ujkl","amount":"10000"}],"gas_limit":"200000","payer":"","granter":""}},"signatures":[]}
        */
        let tx = MsgBuyStorage {
            creator: info.sender.to_string(),
            for_address,
            duration_days: duration,
            bytes,
            payment_denom,
            referral: "execute-buy-storage".to_string(),
        };
        let resp = Response::new()
            .add_message(CosmosMsg::Stargate {
                type_url: "/canine_chain.storage.MsgBuyStorage".to_string(),
                value: to_json_binary(&tx)?,
            })
            .add_attribute("action", "buy_storage");
        
        /*
        custom cosmos msg also fails
        let resp = Response::new()
            .add_message(CosmosMsg::Custom(tx))
            .add_attribute("action", "buy_storage");
        */
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {}

impl CustomMsg for ExecuteMsg {}
