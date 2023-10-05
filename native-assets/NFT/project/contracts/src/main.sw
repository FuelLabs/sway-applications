contract;

mod errors;

use errors::{MintError, SetError};
use src_20::SRC20;
use src_3::SRC3;
use src_7::{SRC7, Metadata};
use token::{
    base::{
        _total_assets, 
        _total_supply,
        _name,
        _symbol,
        _set_name,
        _set_symbol,
        SetTokenAttributes,
    },
    mint::{_mint, _burn},
    metadata::*,
};
use std::{
    call_frames::contract_id, 
    hash::Hash, 
    string::String, 
    storage::storage_string::*
};

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
    /// The metadata associated with a particular asset.
    ///
    /// # Additional Information
    ///
    /// In this NFT contract, there is no metadata provided at compile time. All metadata
    /// is added by users and stored into storage.
    metadata: StorageMetadata = StorageMetadata {},
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
    /// This conforms to the SRC-20 NFT portion of the standard for a maximium
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
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        let asset = AssetId::new(contract_id(), sub_id);
        require(amount == 1, MintError::CannotMintMoreThanOneNFTWithSubId);
        require(storage.total_supply.get(asset).try_read().is_none(), MintError::NFTAlreadyMinted);
        require(storage.total_assets.try_read().unwrap_or(0) + amount <= 100_000, MintError::MaxNFTsMinted);
        
        let _ = _mint(storage.total_assets, storage.total_supply, recipient, sub_id, amount);
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
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        _burn(storage.total_supply, sub_id, amount);
    }
}

impl SRC7 for Contract {
    /// Returns metadata for the corresponding `asset` and `key`.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the metadata.
    /// * `key`: [String] - The key to the specific metadata.
    ///
    /// # Returns
    ///
    /// * [Option<Metadata>] - `Some` metadata that corresponds to the `key` or `None`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src_7::{SRC7, Metadata};
    /// use std::string::String;
    ///
    /// fn foo(contract_id: ContractId, asset: AssetId) {
    ///     let contract_abi = abi(SRC7, contract_id);
    ///     let key = String::from_ascii_str("image");
    ///     let data = contract_abi.metadata(asset, key);
    ///     assert(data.is_some());
    /// }
    /// ```
    #[storage(read)]
    fn metadata(asset: AssetId, key: String) -> Option<Metadata> {
        storage.metadata.get(asset, key)
    }
}

impl SetTokenAttributes for Contract {
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
    /// use token::SetTokenAttributes;
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(asset: AssetId, contract_id: ContractId) {
    ///     let set_abi = abi(SetTokenAttributes, contract_id);
    ///     let src_20_abi = abi(SRC20, contract_id);
    ///     let name = String::from_ascii_str("Ether");
    ///     set_abi.set_name(asset, name);
    ///     assert(src_20_abi.name(asset) == name);
    /// }
    /// ```
    #[storage(write)]
    fn set_name(asset: AssetId, name: String) {
        require(storage.name.get(asset).read_slice().is_none(), SetError::ValueAlreadySet);
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
    /// use token::SetTokenAttributes;
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(asset: AssetId, contract_id: ContractId) {
    ///     let set_abi = abi(SetTokenAttributes, contract_id);
    ///     let src_20_abi = abi(SRC20, contract_id);
    ///     let symbol = String::from_ascii_str("ETH");
    ///     set_abi.set_symbol(asset, symbol);
    ///     assert(src_20_abi.symbol(asset) == symbol);
    /// }
    /// ```
    #[storage(write)]
    fn set_symbol(asset: AssetId, symbol: String) {
        require(storage.symbol.get(asset).read_slice().is_none(), SetError::ValueAlreadySet);
        _set_symbol(storage.symbol, asset, symbol);
    }

    /// This function should never be called.
    /// 
    /// # Additional Information
    ///
    /// NFT decimals are always `0u8` and thus must not be set.
    /// This function is an artifact of the SetTokenAttributes ABI definition, 
    /// but does not have a use in this contract as the decimal value is hardcoded.
    ///
    /// # Reverts
    ///
    /// * When the function is called.
    #[storage(write)]
    fn set_decimals(asset: AssetId, decimals: u8) {
        require(false, SetError::ValueAlreadySet);
    }
}

impl SetTokenMetadata for Contract {
    /// Stores metadata for a specific asset and key pair.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset for the metadata to be stored.
    /// * `key`: [String] - The key for the metadata to be stored.
    /// * `metadata`: [Metadata] - The metadata to be stored.
    ///
    /// # Reverts
    ///
    /// * When the metadata has already been set for an asset.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `2`
    ///
    /// # Example
    ///
    /// ```sway
    /// use src_7::{SRC7, Metadata};
    /// use token::metdata::SetTokenMetadata;
    ///
    /// fn foo(asset: AssetId, key: String, contract_id: ContractId, metadata: Metadata) {
    ///     let set_abi = abi(SetTokenMetadata, contract_id);
    ///     let src_7_abi = abi(SRC7, contract);
    ///     set_abi.set_metadata(storage.metadata, asset, key, metadata);
    ///     assert(src_7_abi.metadata(asset, key) == metadata);
    /// }
    /// ```
    #[storage(read, write)]
    fn set_metadata(asset: AssetId, key: String, metadata: Metadata) {
        require(storage.metadata.get(asset, key).is_none(), SetError::ValueAlreadySet);
        _set_metadata(storage.metadata, asset, key, metadata);
    }
}

// Tests

#[test]
fn test_mint() {
    use std::context::balance_of;
    use std::constants::ZERO_B256;

    let src3_abi = abi(SRC3, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);

    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 0);

    src3_abi.mint(recipient, sub_id, 1);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 1);
}

#[test(should_revert)]
fn test_revert_mint_amount_greater_than_one() {
    use std::constants::ZERO_B256;

    let src3_abi = abi(SRC3, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let amount = 2;

    src3_abi.mint(recipient, sub_id, amount);
}

#[test(should_revert)]
fn test_revert_mint_twice() {
    use std::constants::ZERO_B256;

    let src3_abi = abi(SRC3, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let amount = 1;

    src3_abi.mint(recipient, sub_id, amount);
    src3_abi.mint(recipient, sub_id, amount);
}

#[test]
fn test_burn() {
    use std::context::balance_of;
    use std::constants::ZERO_B256;

    let src3_abi = abi(SRC3, CONTRACT_ID);
    let recipient = Identity::ContractId(ContractId::from(CONTRACT_ID));
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);

    src3_abi.mint(recipient, sub_id, 1);
    assert(balance_of(ContractId::from(CONTRACT_ID), asset_id) == 1);

    src3_abi.burn(sub_id, 1);
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

    src3_abi.mint(recipient, sub_id1, 1);
    assert(src20_abi.total_assets() == 1);

    src3_abi.mint(recipient, sub_id2, 1);
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

    src3_abi.mint(recipient, sub_id, 1);
    assert(src20_abi.total_supply(asset_id).unwrap() == 1);
}

#[test]
fn test_name() {
    use std::constants::ZERO_B256;
    
    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetTokenAttributes, CONTRACT_ID);

    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let name = String::from_ascii_str("Fuel Token");

    assert(src20_abi.name(asset_id).is_none());

    attributes_abi.set_name(asset_id, name);
    assert(src20_abi.name(asset_id).unwrap().as_bytes() == name.as_bytes());
}

#[test(should_revert)]
fn test_revert_set_name_twice() {
    use std::constants::ZERO_B256;
    
    let attributes_abi = abi(SetTokenAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let name = String::from_ascii_str("Fuel Token");

    attributes_abi.set_name(asset_id, name);
    attributes_abi.set_name(asset_id, name);
}

#[test]
fn test_symbol() {
    use std::constants::ZERO_B256;

    let src20_abi = abi(SRC20, CONTRACT_ID);
    let attributes_abi = abi(SetTokenAttributes, CONTRACT_ID);

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

    let attributes_abi = abi(SetTokenAttributes, CONTRACT_ID);
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
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let decimals = 0u8;

    assert(src20_abi.decimals(asset_id).unwrap() == decimals);
}

#[test(should_revert)]
fn test_revert_set_decimals() {
    use std::constants::ZERO_B256;

    let attributes_abi = abi(SetTokenAttributes, CONTRACT_ID);
    let sub_id = ZERO_B256;
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), sub_id);
    let decimals = 0u8;

    attributes_abi.set_decimals(asset_id, decimals);
}

#[test]
fn test_set_metadata() {
    use std::constants::ZERO_B256;

    let data_b256 = 0x0000000000000000000000000000000000000000000000000000000000000001;
    let metadata = Metadata::B256(data_b256);
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), ZERO_B256);
    let src7_abi = abi(SRC7, CONTRACT_ID);
    let set_metadata_abi = abi(SetTokenMetadata, CONTRACT_ID);
    let key = String::from_ascii_str("my_key");
    
    set_metadata_abi.set_metadata(asset_id, key, metadata);

    let returned_metadata = src7_abi.metadata(asset_id, key);
    assert(returned_metadata.is_some());
    assert(returned_metadata.unwrap() == metadata);
}

#[test(should_revert)]
fn test_revert_set_metadata_twice() {
    use std::constants::ZERO_B256;

    let data_b256 = 0x0000000000000000000000000000000000000000000000000000000000000001;
    let metadata = Metadata::B256(data_b256);
    let asset_id = AssetId::new(ContractId::from(CONTRACT_ID), ZERO_B256);
    let set_metadata_abi = abi(SetTokenMetadata, CONTRACT_ID);
    let key = String::from_ascii_str("my_key");
    
    set_metadata_abi.set_metadata(asset_id, key, metadata);
    set_metadata_abi.set_metadata(asset_id, key, metadata);
}
