use cosmwasm_std::{
    attr, coins, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{StakedNFTsResponse, RewardsResponse};
use crate::state::{CONFIG, STAKED_NFTS, USER_STAKES, PENDING_REWARDS, TOTAL_STAKED, LAST_DISTRIBUTION};

pub fn execute_stake_nfts(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Validate NFT collection(s)
    if !config.is_multi_collection && config.allowed_collections.len() != 1 {
        return Err(ContractError::InvalidNFTCollection {});
    }

    // TODO: Implement NFT transfer logic (depends on the specific NFT standard used)

    for token_id in token_ids.iter() {
        STAKED_NFTS.save(deps.storage, (&info.sender, token_id), &crate::state::StakedNFT {
            owner: info.sender.clone(),
            token_id: token_id.clone(),
        })?;
    }

    USER_STAKES.update(deps.storage, &info.sender, |stakes| -> StdResult<_> {
        let mut stakes = stakes.unwrap_or_default();
        stakes.extend(token_ids.clone());
        Ok(stakes)
    })?;

    TOTAL_STAKED.update(deps.storage, |total| -> StdResult<_> {
        Ok(total + token_ids.len() as u64)
    })?;

    distribute_rewards(&mut deps, &env)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "stake_nfts"),
        attr("user", info.sender.to_string()),
        attr("token_ids", token_ids.join(",")),
    ]))
}

pub fn execute_unstake_nfts(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let stakes = USER_STAKES.load(deps.storage, &info.sender)?;
    if stakes.is_empty() {
        return Err(ContractError::NoNFTsStaked {});
    }

    // TODO: Implement NFT transfer back logic

    for token_id in stakes.iter() {
        STAKED_NFTS.remove(deps.storage, (&info.sender, token_id));
    }

    USER_STAKES.remove(deps.storage, &info.sender);

    TOTAL_STAKED.update(deps.storage, |total| -> StdResult<_> {
        Ok(total - stakes.len() as u64)
    })?;

    distribute_rewards(&mut deps, &env)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "unstake_nfts"),
        attr("user", info.sender.to_string()),
        attr("token_ids", stakes.join(",")),
    ]))
}

pub fn execute_claim_rewards(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    distribute_rewards(&mut deps, &env)?;

    let rewards = PENDING_REWARDS.load(deps.storage, &info.sender)?;
    if rewards.is_zero() {
        return Err(ContractError::InsufficientFunds {});
    }

    let config = CONFIG.load(deps.storage)?;
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(rewards.u128(), config.token_denom),
    });

    PENDING_REWARDS.save(deps.storage, &info.sender, &Uint128::zero())?;

    Ok(Response::new()
        .add_message(msg)
        .add_attributes(vec![
            attr("action", "claim_rewards"),
            attr("user", info.sender.to_string()),
            attr("amount", rewards.to_string()),
        ]))
}

fn distribute_rewards(deps: &mut DepsMut, env: &Env) -> StdResult<()> {
    let config = CONFIG.load(deps.storage)?;
    let total_staked = TOTAL_STAKED.load(deps.storage)?;
    let last_distribution = LAST_DISTRIBUTION.load(deps.storage).unwrap_or(0);

    let elapsed = env.block.time.seconds() - last_distribution;
    let total_rewards = config.distribution_rate * Uint128::from(elapsed);

    if total_staked > 0 {
        let reward_per_nft = total_rewards.checked_div(Uint128::from(total_staked)).unwrap_or_default();

        // Collect all updates in a vector
        let mut updates: Vec<(Addr, Uint128)> = vec![];
        STAKED_NFTS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending).for_each(|item| {
            let ((_owner, _token_id), staked_nft) = item.unwrap();
            updates.push((staked_nft.owner, reward_per_nft));
        });

        // Apply updates outside the closure
        for (owner, reward) in updates {
            PENDING_REWARDS.update(deps.storage, &owner, |rewards| -> StdResult<_> {
                Ok(rewards.unwrap_or_default() + reward)
            })?;
        }
    }

    LAST_DISTRIBUTION.save(deps.storage, &env.block.time.seconds())?;

    Ok(())
}

pub fn query_staked_nfts(deps: cosmwasm_std::Deps, address: Addr) -> StdResult<StakedNFTsResponse> {
    let nfts = USER_STAKES.load(deps.storage, &address).unwrap_or_default();
    Ok(StakedNFTsResponse { nfts })
}

pub fn query_pending_rewards(deps: cosmwasm_std::Deps, address: Addr) -> StdResult<RewardsResponse> {
    let rewards = PENDING_REWARDS.load(deps.storage, &address).unwrap_or_default();
    Ok(RewardsResponse { rewards })
}