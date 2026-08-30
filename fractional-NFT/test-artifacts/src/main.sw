contract;

mod errors;

use errors::MintError;
use standards::src20::SRC20;
use standards::src3::SRC3;
use sway_libs::asset::{
    base::{
        _name,
        _set_name,
        _set_symbol,
        _symbol,
        _total_assets,
        _total_supply,
    },
    supply::{
        _burn,
        _mint,
    },
};
use std::{hash::Hash, storage::storage_string::*, string::String};

storage {
    /// The total number of unique assets minted by this contract.
    /// 
    /// Represents the number of NFTs created by this contract.
    total_assets: u64 = 0,
    /// The total supply of tokens for a given asset.
    ///
    /// In this NFT contract, this will always be 1 for any asset, 
    /// as NFTs are unique and cannot be duplicated.
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    /// The name associated with each asset.
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The symbol associated with each asset.
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
}

impl SRC20 for Contract {
    /// Returns the total number of individual NFTs minted by this contract.
    ///
    /// # Returns
    ///
    /// * [u64] - The total number of NFTs minted.
    #[storage(read)]
    fn total_assets() -> u64 {
        _total_assets(storage.total_assets)
    }

    /// Returns the total supply of tokens for a specific asset.
    ///
    /// NFTs have a supply of at most 1.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset for which the total supply is queried.
    ///
    /// # Returns
    ///
    /// * [Option<u64>] - The total supply of tokens for `asset`.
    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        _total_supply(storage.total_supply, asset)
    }

    /// Returns the name of a given asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset whose name is being queried.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The name of `asset`.
    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        _name(storage.name, asset)
    }

    /// Returns the symbol of a given asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset whose symbol is being queried.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The symbol of `asset`.
    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        _symbol(storage.symbol, asset)
    }

    /// Returns the number of decimals the asset uses.
    ///
    /// NFTs typically have 0 decimals since they are indivisible.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset whose decimal precision is being queried.
    ///
    /// # Returns
    ///
    /// * [Option<u8>] - The decimal precision used by `asset`.
    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        Some(0u8)
    }
}

impl SRC3 for Contract {
    /// Mints a new NFT using a unique sub-identifier.
    ///
    /// This contract adheres to the SRC-20 NFT standard, allowing a maximum mint amount of 1 token per asset.
    ///
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The recipient of the minted token.
    /// * `sub_id`: [SubId] - A unique identifier for the newly minted token.
    /// * `amount`: [u64] - The quantity of tokens to mint. Must be exactly 1 for NFTs.
    ///
    /// # Reverts
    ///
    /// * When `amount` is greater than 1.
    /// * When the asset has already been minted.
    /// * When more than 100,000 NFTs have been minted.
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        let asset = AssetId::new(ContractId::this(), sub_id);
        require(amount == 1, MintError::CannotMintMoreThanOneNFTWithSubId);
        require(
            storage
                .total_supply
                .get(asset)
                .try_read()
                .is_none(),
            MintError::NFTAlreadyMinted,
        );
        require(
            storage
                .total_assets
                .try_read()
                .unwrap_or(0) + amount <= 100_000,
            MintError::MaxNFTsMinted,
        );
        let _ = _mint(
            storage
                .total_assets,
            storage
                .total_supply,
            recipient,
            sub_id,
            amount,
        );
    }

    /// Burns a specified NFT using the given sub-identifier.
    ///
    /// NFTs can be burned, removing them from circulation.
    ///
    /// # Arguments
    ///
    /// * `sub_id`: [SubId] - The unique sub-identifier of the token to be burned.
    /// * `amount`: [u64] - The quantity of tokens to burn. For NFTs, this should typically be 1.
    #[payable]
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        _burn(storage.total_supply, sub_id, amount);
    }
}
