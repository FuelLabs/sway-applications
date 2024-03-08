contract;

mod errors;

use errors::{MintError, SetError};
use src20::SRC20;
use src3::SRC3;
use asset::{
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
    mint::{
        _burn,
        _mint,
    },
};
use std::{call_frames::contract_id, hash::Hash, storage::storage_string::*, string::String};

storage {
    /// The total number of unique assets minted by this contract.
    total_assets: u64 = 0,
    /// The total number of coins minted for a particular asset.
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    /// The name associated with a particular asset.
    name: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The symbol associated with a particular asset.
    symbol: StorageMap<AssetId, StorageString> = StorageMap {},
    /// The decimals associated with a particular asset.
    decimals: StorageMap<AssetId, u8> = StorageMap {},
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
    /// use src20::SRC20;
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
    /// use src20::SRC20;
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
    /// use src20::SRC20;
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
    /// use src20::SRC20;
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
    /// use src20::SRC20;
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
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The user to which the newly minted assets are transferred to.
    /// * `sub_id`: [SubId] - The sub-identifier of the newly minted asset.
    /// * `amount`: [u64] - The quantity of coins to mint.
    ///
    /// # Reverts
    ///
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
    /// use src3::SRC3;
    ///
    /// fn foo(contract: ContractId) {
    ///     let contract_abi = abi(SR3, contract);
    ///     contract_abi.mint(Identity::ContractId(this_contract()), ZERO_B256, 100);
    /// }
    /// ```
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        let asset = AssetId::new(contract_id(), sub_id);
        require(
            storage
                .total_supply
                .get(asset)
                .try_read()
                .unwrap_or(0) + amount < 100_000_000,
            MintError::MaxMinted,
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
    /// Burns assets sent with the given `sub_id`.
    ///
    /// # Additional Information
    ///
    /// NOTE: The sha-256 hash of `(ContractId, SubId)` must match the `AssetId` where `ContractId` is the id of
    /// the implementing contract and `SubId` is the given `sub_id` argument.
    ///
    /// # Arguments
    ///
    /// * `sub_id`: [SubId] - The sub-identifier of the asset to burn.
    /// * `amount`: [u64] - The quantity of coins to burn.
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
    /// fn foo(contract: ContractId, asset_id: AssetId) {
    ///     let contract_abi = abi(SR3, contract);
    ///     contract_abi {
    ///         gas: 10000,
    ///         coins: 100,
    ///         asset_id: AssetId,
    ///     }.burn(ZERO_B256, 100);
    /// }
    /// ```
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        _burn(storage.total_supply, sub_id, amount);
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
    /// use asset::SetAssetAttributes;
    /// use src20::SRC20;
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
        require(
            storage
                .name
                .get(asset)
                .read_slice()
                .is_none(),
            SetError::ValueAlreadySet,
        );
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
    /// use src20::SRC20;
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
        require(
            storage
                .symbol
                .get(asset)
                .read_slice()
                .is_none(),
            SetError::ValueAlreadySet,
        );
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
    /// use asset::SetAssetAttributes;
    /// use src20::SRC20;
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
        require(
            storage
                .decimals
                .get(asset)
                .try_read()
                .is_none(),
            SetError::ValueAlreadySet,
        );
        _set_decimals(storage.decimals, asset, decimals);
    }
}

#[test]
fn test_mint() {
    use std::context::balance_of;
    use std::constants::ZERO_B256;
    let src3_abi = abi(SRC3, CONTRACT_ID);
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 0);
    src3_abi.mint(recipient, sub_id, 100);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 100);
}
#[test(should_revert)]
fn test_revert_mint_amount_greater_than_max() {
    use std::constants::ZERO_B256;
    let src3_abi = abi(SRC3, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let amount = 100_000_001;
    src3_abi.mint(recipient, sub_id, amount);
}
#[test]
fn test_burn() {
    use std::context::balance_of;
    use std::constants::ZERO_B256;
    let src3_abi = abi(SRC3, CONTRACT_ID);
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    src3_abi.mint(recipient, sub_id, 100);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 100);
    src3_abi.burn(sub_id, 100);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 0);
}
#[test]
fn test_total_assets() {
    let src3_abi = abi(SRC3, CONTRACT_ID);
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id1 = 0x0000000000000000000000000000000000000000000000000000000000000001;
    let sub_id2 = 0x0000000000000000000000000000000000000000000000000000000000000002;
    assert(src20_abi.total_assets() == 0);
    src3_abi.mint(recipient, sub_id1, 100);
    assert(src20_abi.total_assets() == 1);
    src3_abi.mint(recipient, sub_id2, 100);
    assert(src20_abi.total_assets() == 2);
}
#[test]
fn test_total_supply() {
    use std::constants::ZERO_B256;
    let src3_abi = abi(SRC3, CONTRACT_ID);
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    assert(src20_abi.total_supply(asset_id).is_none());
    src3_abi.mint(recipient, sub_id, 100);
    assert(src20_abi.total_supply(asset_id).unwrap() == 100);
}
#[test]
fn test_name() {
    use std::constants::ZERO_B256;
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let name = String::from_ascii_str("Fuel Asset");
    assert(src20_abi.name(asset_id).is_none());
    attributes_abi.set_name(asset_id, name);
    assert(src20_abi.name(asset_id).unwrap().as_bytes() == name.as_bytes());
}
#[test(should_revert)]
fn test_revert_set_name_twice() {
    use std::constants::ZERO_B256;
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let name = String::from_ascii_str("Fuel Asset");
    attributes_abi.set_name(asset_id, name);
    attributes_abi.set_name(asset_id, name);
}
#[test]
fn test_symbol() {
    use std::constants::ZERO_B256;
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let symbol = String::from_ascii_str("FUEL");
    assert(src20_abi.symbol(asset_id).is_none());
    attributes_abi.set_symbol(asset_id, symbol);
    assert(src20_abi.symbol(asset_id).unwrap().as_bytes() == symbol.as_bytes());
}
#[test(should_revert)]
fn test_revert_set_symbol_twice() {
    use std::constants::ZERO_B256;
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let symbol = String::from_ascii_str("FUEL");
    attributes_abi.set_symbol(asset_id, symbol);
    attributes_abi.set_symbol(asset_id, symbol);
}
#[test]
fn test_decimals() {
    use std::constants::ZERO_B256;
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let decimals = 8u8;
    assert(src20_abi.decimals(asset_id).is_none());
    attributes_abi.set_decimals(asset_id, decimals);
    assert(src20_abi.decimals(asset_id).unwrap() == decimals);
}
#[test(should_revert)]
fn test_revert_set_decimals_twice() {
    use std::constants::ZERO_B256;
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetAssetAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let decimals = 8u8;
    attributes_abi.set_decimals(asset_id, decimals);
    attributes_abi.set_decimals(asset_id, decimals);
}
