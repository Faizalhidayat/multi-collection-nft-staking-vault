use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid NFT collection")]
    InvalidNFTCollection {},

    #[error("No NFTs staked")]
    NoNFTsStaked {},

    #[error("Insufficient funds")]
    InsufficientFunds {},
}