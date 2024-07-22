use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub creator: Addr,
    pub is_multi_collection: bool,
    pub allowed_collections: Vec<Addr>,
    pub token_denom: String,
    pub distribution_rate: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    StakeNFTs { token_ids: Vec<String> },
    UnstakeNFTs {},
    AddTokens { amount: Uint128 },
    ClaimRewards {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StakedNFTsResponse)]
    StakedNFTs { address: Addr },
    #[returns(RewardsResponse)]
    PendingRewards { address: Addr },
}

#[cw_serde]
pub struct StakedNFTsResponse {
    pub nfts: Vec<String>,
}

#[cw_serde]
pub struct RewardsResponse {
    pub rewards: Uint128,
}