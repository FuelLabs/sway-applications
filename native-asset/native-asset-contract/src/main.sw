contract;

mod errors;
mod interface;

use errors::{AmountError, MintError, SetError};
use standards::{src20::SRC20, src3::SRC3, src5::{SRC5, State},};
use sway_libs::{
    asset::{
        base::{
            _decimals,
            _name,
            _set_decimals,
            _set_name,
            _set_symbol,
            _symbol,
            _total_assets,
            _total_supply,
            SetAssetAttributes,
        },
        supply::{
            _burn,
            _mint,
        },
    },
    ownership::{
        _owner,
        initialize_ownership,
        only_owner,
    },
};
use interface::Constructor;
use std::{context::msg_amount, hash::Hash, storage::storage_string::*, string::String,};

storage {
    /// The total number of unique assets minted by this contract.
    total_assets: u64 = 0,
    /// The current total number of coins minted for a particular asset.
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    /// The name associated with a particular asset.
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The symbol associated with a particular asset.
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The decimals associated with a particular asset.
    decimals: StorageMap<AssetId, u8> = StorageMap {},
    /// The total number of coins ever minted for an asset.
    cumulative_supply: StorageMap<AssetId, u64> = StorageMap {},
}

configurable {
    /// The maximum supply allowed for any single asset.
    MAX_SUPPLY: u64 = 100_000_000,
}

impl SRC20 for Contract {
    /// Returns the total number of individual assets for a contract.
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
    /// use standards::src20::SRC20;
    ///
    /// fn foo(contract: ContractId) {
    ///     let contract_abi = abi(SRC20, contract);
    ///     let total_assets = contract_abi.total_assets().unwrap();
    ///     assert(total_assets != 0);
    /// }
    /// ```
    #[storage(read)]
    fn total_assets() -> u64 {
        _total_assets(storage.total_assets)
    }

    /// Returns the total supply of coins for an asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the total supply.
    ///
    /// # Returns
    ///
    /// * [Option<u64>] - The total supply of coins for `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src20::SRC20;
    ///
    /// fn foo(contract: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract);
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
    /// use standards::src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(contract: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract);
    ///     let name = contract_abi.name(asset);
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
    /// use standards::src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(contract: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC20, contract);
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
    /// e.g. 8, means to divide the coin amount by 100000000 to get its user interface representation.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the decimals.
    ///
    /// # Returns
    ///
    /// * [Option<u8>] - The decimal precision used by `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src20::SRC20;
    ///
    /// fn foo(contract: ContractId, asset: AssedId) {
    ///     let contract_abi = abi(SRC20, contract);
    ///     let decimals = contract_abi.decimals(asset).unwrap();
    ///     assert(decimals == 8u8);
    /// }
    /// ```
    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        _decimals(storage.decimals, asset)
    }
}

impl SRC3 for Contract {
    /// Mints new assets using the `sub_id` sub-identifier.
    ///
    /// # Additional Information
    ///
    ///
    /// NOTE: This function will only ever let MAX_SUPPLY coins be minted, even if other coins are burned reducing the total supply.
    ///
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The user to which the newly minted assets are transferred to.
    /// * `sub_id`: [SubId] - The sub-identifier of the newly minted asset.
    /// * `amount`: [u64] - The quantity of coins to mint.
    ///
    /// # Reverts
    ///
    /// * When the caller is not the contract owner.
    /// * When `sub_id` is `None`.
    /// * When more than 100,000,000 coins have been minted.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src3::SRC3;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(contract: ContractId, identity: Identity) {
    ///     let contract_abi = abi(SR3, contract);
    ///     contract_abi.mint(identity, DEFAULT_SUB_ID, 100);
    /// }
    /// ```
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: Option<SubId>, amount: u64) {
        only_owner();

        require(sub_id.is_some(), MintError::SubIdNone);
        let asset = AssetId::new(ContractId::this(), sub_id.unwrap());
        let cumulative_supply = storage.cumulative_supply.get(asset).try_read().unwrap_or(0);
        require(
            cumulative_supply + amount <= MAX_SUPPLY,
            MintError::MaxMinted,
        );
        storage
            .cumulative_supply
            .insert(asset, cumulative_supply + amount);

        // NOTE: TotalSupplyEvent is emitted by _mint()
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
    /// Burns assets sent with the given `sub_id`.
    ///
    /// # Additional Information
    ///
    /// NOTE: The sha-256 hash of `(ContractId, SubId)` must match the `AssetId` where `ContractId` is the id of the implementing contract and `SubId` is the given `sub_id` argument.
    /// NOTE: This function reduces the total supply.
    ///
    /// # Arguments
    ///
    /// * `sub_id`: [SubId] - The sub-identifier of the asset to burn.
    /// * `amount`: [u64] - The quantity of coins to burn.
    ///
    /// # Reverts
    ///
    /// * When the `amount` provided and transaction amount do not match.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src3::SRC3;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(contract: ContractId) {
    ///     let contract_abi = abi(SR3, contract);
    ///     let asset_id = AssetId::new(contract, DEFAULT_SUB_ID);
    ///     contract_abi {
    ///         gas: 10000,
    ///         coins: 100,
    ///         asset_id: AssetId,
    ///     }.burn(DEFAULT_SUB_ID, 100);
    /// }
    /// ```
    #[payable]
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        require(msg_amount() == amount, AmountError::AmountMismatch);
        // NOTE: TotalSupplyEvent is emitted by _burn()
        _burn(storage.total_supply, sub_id, amount);
    }
}

impl SRC5 for Contract {
    /// Returns the owner.
    ///
    /// # Return Values
    ///
    /// * [State] - Represents the state of ownership for this contract.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src5::SRC5;
    ///
    /// fn foo(contract_id: ContractId) {
    ///     let ownership_abi = abi(contract_id, SRC_5);
    ///
    ///     match ownership_abi.owner() {
    ///         State::Uninitalized => log("The ownership is uninitalized"),
    ///         _ => log("This example will never reach this statement"),
    ///     }
    /// }
    /// ```
    #[storage(read)]
    fn owner() -> State {
        _owner()
    }
}

impl SetAssetAttributes for Contract {
    /// Sets the name of an asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to set the name.
    /// * `name`: [String] - The name of the asset.
    ///
    /// # Reverts
    ///
    /// * When the caller is not the contract owner.
    /// * When the name has already been set for an asset.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `2`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use sway_libs::asset::SetAssetAttributes;
    /// use standards::src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(asset: AssetId) {
    ///     let set_abi = abi(SetAssetAttributes, contract_id);
    ///     let src_20_abi = abi(SRC20, contract_id);
    ///     let name = String::from_ascii_str("Ether");
    ///     set_abi.set_name(storage.name, asset, name);
    ///     assert(src_20_abi.name(asset) == name);
    /// }
    /// ```
    #[storage(write)]
    fn set_name(asset: AssetId, name: String) {
        only_owner();

        require(
            storage
                .name
                .get(asset)
                .read_slice()
                .is_none(),
            SetError::ValueAlreadySet,
        );
        // NOTE: SetNameEvent is emitted by _set_name()
        _set_name(storage.name, asset, name);
    }

    /// Sets the symbol of an asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to set the symbol.
    /// * `symbol`: [String] - The symbol of the asset.
    ///
    /// # Reverts
    ///
    /// * When the caller is not the contract owner.
    /// * When the symbol has already been set for an asset.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `2`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use asset::SetAssetAttributes;
    /// use standards::src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(asset: AssetId) {
    ///     let set_abi = abi(SetAssetAttributes, contract_id);
    ///     let src_20_abi = abi(SRC20, contract_id);
    ///     let symbol = String::from_ascii_str("ETH");
    ///     set_abi.set_symbol(storage.name, asset, symbol);
    ///     assert(src_20_abi.symbol(asset) == symbol);
    /// }
    /// ```
    #[storage(write)]
    fn set_symbol(asset: AssetId, symbol: String) {
        only_owner();

        require(
            storage
                .symbol
                .get(asset)
                .read_slice()
                .is_none(),
            SetError::ValueAlreadySet,
        );
        // NOTE: SetSymbolEvent is emitted by _set_symbol()
        _set_symbol(storage.symbol, asset, symbol);
    }

    /// Sets the decimals of an asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to set the decimals.
    /// * `decimal`: [u8] - The decimals of the asset.
    ///
    /// # Reverts
    ///
    /// * When the caller is not the contract owner.
    /// * When the decimals has already been set for an asset.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use sway_libs::asset::SetAssetAttributes;
    /// use standards::src20::SRC20;
    ///
    /// fn foo(asset: AssetId) {
    ///     let decimals = 8u8;
    ///     let set_abi = abi(SetAssetAttributes, contract_id);
    ///     let src_20_abi = abi(SRC20, contract_id);
    ///     set_abi.set_decimals(asset, decimals);
    ///     assert(src_20_abi.decimals(asset) == decimals);
    /// }
    /// ```
    #[storage(write)]
    fn set_decimals(asset: AssetId, decimals: u8) {
        only_owner();

        require(
            storage
                .decimals
                .get(asset)
                .try_read()
                .is_none(),
            SetError::ValueAlreadySet,
        );
        // NOTE: SetDecimalsEvent is emitted by _set_decimals()
        _set_decimals(storage.decimals, asset, decimals);
    }
}

impl Constructor for Contract {
    /// Sets the defaults for the contract.
    ///
    /// # Arguments
    ///
    /// * `owner`: [Identity] - The `Identity` that will be the first owner.
    ///
    /// # Reverts
    ///
    /// * When ownership has been set before.
    ///
    /// # Number of Storage Acesses
    ///
    /// * Reads: `1`
    /// * Write: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use standards::src5::SRC5;
    /// use native_asset::Constructor;
    ///
    /// fn foo(contract: ContractId, owner: Identity) {
    ///     let src_5_abi = abi(SRC5, contract.bits());
    ///     assert(src_5_abi.owner() == State::Uninitialized);
    ///
    ///     let constructor_abi = abi(Constructor, contract.bits());
    ///     constructor_abi.constructor(owner);
    ///     assert(src_5_abi.owner() == State::Initialized(owner));
    /// }
    /// ```
    #[storage(read, write)]
    fn constructor(owner: Identity) {
        initialize_ownership(owner);
    }
}
