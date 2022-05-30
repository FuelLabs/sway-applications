contract;

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, Sender, msg_sender},
    constants::NATIVE_ASSET_ID,
    result::*,
    revert::revert,
    storage::StorageMap,
};

abi NFT {
    fn allowMint(minter: Address) -> bool;
    fn approve(to: Address, token_id: b256) -> bool;
    fn balance_of(owner: Address) -> u64;
    fn burn(token_id: b256) -> bool ;
    fn constructor(owner: Address, access_control: bool, token_count: u64) -> bool;
    fn getApproved(token_id: b256) -> Address;
    fn getTotalSupply() -> u64;
    fn isApprovedForAll(owner: Address, operator: Address) -> bool;
    fn mint(to: Address, amount: u64) -> bool ;
    fn owner_of(token_id: b256) -> Address;
    fn setApprovalForAll(owner: Address, operator: Address) -> bool;
    fn transferFrom(from: Address, to: Address, token_id: b256) -> bool;
}

enum Error {
    CannotReinitialize: (),
    InputAddressCannotBeZero: (),
    NFTNotInitalized: (),
    TokenCountCannotBeZero: (),
}

struct MetaData {
    // NFT Metadata
}

storage {
    access_control: bool,
    approvals: StorageMap<b256, Address>,
    balances: StorageMap<Address, u64>,
    metaData: StorageMap<b256, MetaData>,
    minters: StorageMap<Address, bool>,
    operatorApproval: StorageMap<Address, Address>,
    owners: StorageMap<Address, b256>,
    state: u64,
    token_count: u64,
}

impl NFT for Contract {
    fn allowMint(minter: Address) -> bool {
        true
    }

    fn approve(to: Address, token_id: b256) -> bool {
        true
    }

    fn balance_of(owner: Address) -> u64 {
        0
    }

    fn burn(token_id: b256) -> bool {
        true
    }

    /// Constructor for the NFT
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has already been called
    /// - The owner is the 0 address
    /// - The token count is 0
    fn constructor(owner: Address, access_control: bool, token_count: u64) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(owner.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        require(token_count != 0, Error::TokenCountCannotBeZero);

        storage.approvals = ~StorageMap::new::<b256, Address>();
        storage.balances = ~StorageMap::new::<Address, u64>();
        storage.metaData = ~StorageMap::new::<b256, MetaData>();
        storage.minters =  ~StorageMap::new::<Address, bool>();
        storage.operatorApproval = ~StorageMap::new::<Address, Address>();
        storage.owners = ~StorageMap::new::<Address, b256>();

        storage.token_count = token_count;
        storage.access_control = access_control;
        storage.minters.insert(owner, true);

        true
    }

    fn getApproved(token_id: b256) -> Address {
        ~Address::from(NATIVE_ASSET_ID)
    }

    fn getTotalSupply() -> u64 {
        0
    }

    fn isApprovedForAll(owner: Address, operator: Address) -> bool {
        true
    }

    fn mint(to: Address, amount: u64) -> bool {
        true
    }

    fn owner_of(token_id: b256) -> Address {
        ~Address::from(NATIVE_ASSET_ID)
    }

    fn setApprovalForAll(owner: Address, operator: Address) -> bool {
        true
    }

    fn transferFrom(from: Address, to: Address, token_id: b256) -> bool {
        true
    }
}
