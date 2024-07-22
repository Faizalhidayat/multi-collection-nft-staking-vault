use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub creator: Addr,
    pub is_multi_collection: bool,
    pub allowed_collections: Vec<Addr>,
    pub token_denom: String,
    pub distribution_rate: Uint128,
}

#[cw_serde]
pub struct StakedNFT {
    pub owner: Addr,
    pub token_id: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOTAL_STAKED: Item<u64> = Item::new("total_staked");
pub const STAKED_NFTS: Map<(&Addr, &str), StakedNFT> = Map::new("staked_nfts");
pub const USER_STAKES: Map<&Addr, Vec<String>> = Map::new("user_stakes");
pub const PENDING_REWARDS: Map<&Addr, Uint128> = Map::new("pending_rewards");
pub const LAST_DISTRIBUTION: Item<u64> = Item::new("last_distribution");