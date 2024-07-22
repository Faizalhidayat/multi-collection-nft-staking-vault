use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::error::ContractError;
use crate::state::CONFIG;

pub fn execute_add_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.creator {
        return Err(ContractError::Unauthorized {});
    }

    // Validate that the sent funds match the specified amount and denom
    let sent_funds = info.funds.iter().find(|coin| coin.denom == config.token_denom);
    if sent_funds.is_none() || sent_funds.unwrap().amount != amount {
        return Err(ContractError::InsufficientFunds {});
    }

    Ok(Response::new().add_attributes(vec![
        ("action", "add_tokens"),
        ("amount", &amount.to_string()),
    ]))
}