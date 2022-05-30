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
    fn allow_mint(minter: Address) -> bool;
    fn approve(to: Address, token_id: b256) -> bool;
    fn balance_of(owner: Address) -> u64;
    fn burn(token_id: b256) -> bool ;
    fn constructor(owner: Address, access_control: bool, token_count: u64) -> bool;
    fn get_approved(token_id: b256) -> Address;
    fn get_total_supply() -> u64;
    fn is_approved_for_all(owner: Address, operator: Address) -> bool;
    fn mint(to: Address, amount: u64) -> bool ;
    fn owner_of(token_id: b256) -> Address;
    fn set_approval_for_all(owner: Address, operator: Address) -> bool;
    fn transfer_from(from: Address, to: Address, token_id: b256) -> bool;
}

enum Error {
    AccessControlNotSet: (),
    AddressAlreadyGivenAccess: (),
    AddressAlreadyGivenApproval: (),
    CannotReinitialize: (),
    InputAddressCannotBeZero: (),
    NFTNotInitalized: (),
    SenderDoesNotHaveAccessControl: (),
    SenderNotOwner: (),
    TokenCountCannotBeZero: (),
}

struct MetaData {
    // NFT Metadata
    owner: Address,
    approved: Address,
}

storage {
    access_control: bool,
    access_control_address: Address,
    balances: StorageMap<Address, u64>,
    meta_data: StorageMap<b256, MetaData>,
    allowed_minters: StorageMap<Address, bool>,
    operator_approval: StorageMap<Address, Address>,
    owners: StorageMap<Address, b256>,
    state: u64,
    token_count: u64,
}

impl NFT for Contract {

    /// Allows access to mint any NFT
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    /// - The NFT contract does not have access control set
    /// - The address provided is the 0 address
    /// - The address has already been allowed access
    /// - The address is not the access control address
    fn allow_mint(minter: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(storage.access_control, Error::AccessControlNotSet);
        require(minter.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        require(storage.allowed_minters.get(minter) == false, Error::AddressAlreadyGivenAccess);
        
        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            require(storage.access_control_address == address, Error::SenderDoesNotHaveAccessControl);
            storage.allowed_minters.insert(minter, true);
        } else {
            revert(0);
        }

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
        storage.meta_data = ~StorageMap::new::<b256, MetaData>();
        storage.allowed_minters =  ~StorageMap::new::<Address, bool>();
        storage.operator_approval = ~StorageMap::new::<Address, Address>();
        storage.owners = ~StorageMap::new::<Address, b256>();

        storage.token_count = token_count;
        storage.access_control = access_control;
        storage.access_control_address = owner;

        true
    }

    /// Returns the approved address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn get_approved(token_id: b256) -> Address {
        require(storage.state != 0, Error::NFTNotInitalized);

        let meta_data: MetaData = storage.meta_data.get(token_id);
        meta_data.approved
    }

    /// Returns the total supply for the NFT contract
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn get_total_supply() -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
        storage.token_count
    }

    /// Returns whether the address is approved for all tokens
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    fn is_approved_for_all(owner: Address, operator: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);

        let address: Address = storage.operator_approval.get(owner);
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

        let meta_data: MetaData = storage.meta_data.get(token_id);
        meta_data.owner
    }

    fn set_approval_for_all(owner: Address, operator: Address) -> bool {
        true
    }

    fn transfer_from(from: Address, to: Address, token_id: b256) -> bool {
        true
    }
}
