# Multi-Collection NFT Staking Vault

This project implements a smart contract system for staking NFTs from multiple collections and distributing rewards in IBC tokens.

## Features

- Stake NFTs from multiple contracts or a single contract
- Unstake all NFTs at once
- Create vaults with customizable settings
- Distribute IBC tokens as rewards at a specified rate
- Allow creators to add tokens but not withdraw them


## Building

To build the contract, run:
cargo build

## Testing

To run the tests, use:
cargo test

## Deploying

1. Compile the contract:
cargo wasm

2. Optimize the wasm binary:
docker run --rm -v "$(pwd)":/code 
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target 
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry 
cosmwasm/rust-optimizer:0.12.6

3. Deploy the optimized wasm file to your chosen CosmWasm-enabled blockchain.

## Usage

1. Instantiate the contract with the desired configuration.
2. Use the `StakeNFTs` message to stake NFTs.
3. Use the `UnstakeNFTs` message to unstake all NFTs.
4. Use the `AddTokens` message to add more reward tokens.
5. Use the `ClaimRewards` message to claim accumulated rewards.

## License

This project is licensed under the MIT License.