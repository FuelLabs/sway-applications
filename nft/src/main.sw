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
    owner: Address,
    approved: Address,
}

storage {
    access_control: bool,
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

    /// Returns the balance of the specified owner
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn balance_of(owner: Address) -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
        storage.balances.get(owner)
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

    /// Returns the approved address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn getApproved(token_id: b256) -> Address {
        require(storage.state != 0, Error::NFTNotInitalized);

        let metaData: MetaData = storage.metaData.get(token_id);
        metaData.approved
    }

    /// Returns the total supply for the NFT contract
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn getTotalSupply() -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
        storage.token_count
    }

    /// Returns whether the address is approved for all tokens
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn isApprovedForAll(owner: Address, operator: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);

        let address: Address = storage.operatorApproval.get(owner);
        // There has to be a better way to do this in sway
        // Looking for something like 'case ? true : false' in C++
        if address.value == operator.value {
            true
        }
        else
        {
            false
        }
    }

    fn mint(to: Address, amount: u64) -> bool {
        true
    }

    /// Returns the owner of a given token id
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn owner_of(token_id: b256) -> Address {
        require(storage.state != 0, Error::NFTNotInitalized);

        let metaData: MetaData = storage.metaData.get(token_id);
        metaData.owner
    }

    fn setApprovalForAll(owner: Address, operator: Address) -> bool {
        true
    }

    fn transferFrom(from: Address, to: Address, token_id: b256) -> bool {
        true
    }
}
