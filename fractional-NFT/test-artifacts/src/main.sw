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
    /// # Additional Information
    ///
    /// This is the number of NFTs that have been minted.
    total_assets: u64 = 0,
    /// The total number of tokens minted for a particular asset.
    ///
    /// # Additional Information
    ///
    /// This should always be 1 for any asset as this is an NFT contract.
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    /// The name associated with a particular asset.
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The symbol associated with a particular asset.
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
}

impl SRC20 for Contract {
    /// Returns the total number of individual NFTs for this contract.
    ///
    /// # Returns
    ///
    /// * [u64] - The number of assets that this contract has minted.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(contract_id: ContractId) {
    ///     let contract_abi = abi(SRC20, contract_id);
    ///     let total_assets = contract_abi.total_assets();
    ///     assert(total_assets != 0);
    /// }
    /// ```
    #[storage(read)]
    fn total_assets() -> u64 {
        _total_assets(storage.total_assets)
    }

    /// Returns the total supply of tokens for an asset.
    ///
    /// # Additional Information
    ///
    /// This must always be at most 1 for NFTs.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the total supply.
    ///
    /// # Returns
    ///
    /// * [Option<u64>] - The total supply of tokens for `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(contract_id: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract_id);
    ///     let total_supply = contract_abi.total_supply(asset).unwrap();
    ///     assert(total_supply == 1);
    /// }
    /// ```
    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        _total_supply(storage.total_supply, asset)
    }

    /// Returns the name of the asset, such as “Ether”.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the name.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The name of `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(contract_ic: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract_id);
    ///     let name = contract_abi.name(asset).unwrap();
    ///     assert(name.len() != 0);
    /// }
    /// ```
    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        _name(storage.name, asset)
    }
    /// Returns the symbol of the asset, such as “ETH”.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the symbol.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The symbol of `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(contract_id: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract_id);
    ///     let symbol = contract_abi.symbol(asset).unwrap();
    ///     assert(symbol.len() != 0);
    /// }
    /// ```
    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        _symbol(storage.symbol, asset)
    }
    /// Returns the number of decimals the asset uses.
    ///
    /// # Additional Information
    ///
    /// The standardized decimals for NFTs is 0u8.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the decimals.
    ///
    /// # Returns
    ///
    /// * [Option<u8>] - The decimal precision used by `asset`.
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(contract_id: ContractId, asset: AssedId) {
    ///     let contract_abi = abi(SRC20, contract_id);
    ///     let decimals = contract_abi.decimals(asset).unwrap();
    ///     assert(decimals == 0u8);
    /// }
    /// ```
    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        Some(0u8)
    }
}

impl SRC3 for Contract {
    /// Mints new tokens using the `sub_id` sub-identifier.
    ///
    /// # Additional Information
    ///
    /// This conforms to the SRC-20 NFT portion of the standard for a maximum
    /// mint amount of 1 token per asset.
    ///
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The user to which the newly minted tokens are transferred to.
    /// * `sub_id`: [SubId] - The sub-identifier of the newly minted token.
    /// * `amount`: [u64] - The quantity of tokens to mint.
    ///
    /// # Reverts
    ///
    /// * When amount is greater than one.
    /// * When the asset has already been minted.
    /// * When more than 100,000 NFTs have been minted.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src3::SRC3;
    ///
    /// fn foo(contract_id: ContractId) {
    ///     let contract_abi = abi(SR3, contract_id);
    ///     contract_abi.mint(Identity::ContractId(this_contract()), ZERO_B256, 1);
    /// }
    /// ```
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: Option<SubId>, amount: u64) {
        let asset = AssetId::new(ContractId::this(), sub_id.unwrap());
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
            sub_id
                .unwrap(),
            amount,
        );
    }
    /// Burns tokens sent with the given `sub_id`.
    ///
    /// # Additional Information
    ///
    /// NOTE: The sha-256 hash of `(ContractId, SubId)` must match the `AssetId` where `ContractId` is the id of
    /// the implementing contract and `SubId` is the given `sub_id` argument.
    ///
    /// # Arguments
    ///
    /// * `sub_id`: [SubId] - The sub-identifier of the token to burn.
    /// * `amount`: [u64] - The quantity of tokens to burn.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src3::SRC3;
    ///
    /// fn foo(contract_id: ContractId, asset_id: AssetId) {
    ///     let contract_abi = abi(SR3, contract_id);
    ///     contract_abi.burn {
    ///         gas: 10000,
    ///         coins: 1,
    ///         asset_id: AssetId,
    ///     } (ZERO_B256, 1);
    /// }
    /// ```
    #[payable]
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        _burn(storage.total_supply, sub_id, amount);
    }
}
