use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::vault;
use crate::instantiator;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        creator: msg.creator,
        is_multi_collection: msg.is_multi_collection,
        allowed_collections: msg.allowed_collections,
        token_denom: msg.token_denom,
        distribution_rate: msg.distribution_rate,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StakeNFTs { token_ids } => vault::execute_stake_nfts(deps, env, info, token_ids),
        ExecuteMsg::UnstakeNFTs {} => vault::execute_unstake_nfts(deps, env, info),
        ExecuteMsg::AddTokens { amount } => instantiator::execute_add_tokens(deps, env, info, amount),
        ExecuteMsg::ClaimRewards {} => vault::execute_claim_rewards(deps, env, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::StakedNFTs { address } => to_binary(&vault::query_staked_nfts(deps, address)?),
        QueryMsg::PendingRewards { address } => to_binary(&vault::query_pending_rewards(deps, address)?),
    }
}