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
    SenderNotOwnerOrApproved: (),
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
    operator_approval: StorageMap<Address, StorageMap<Address, bool>>,
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

    /// Gives approval to the to address to transfer the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    /// - The address provided is the 0 address
    /// - The address has already been approved
    /// - The sender is not the owner
    fn approve(to: Address, token_id: b256) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(to.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);

        let mut meta_data = storage.meta_data.get(token_id);
        require(meta_data.approved != to, Error::AddressAlreadyGivenApproval);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            require(meta_data.owner == address, Error::SenderNotOwner);

            meta_data.approved = to;
            storage.meta_data.insert(token_id, meta_data);
        } else {
            revert(0);
        };

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
        storage.operator_approval = ~StorageMap::new::<Address, StorageMap<Address, bool>>();

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

        let operator_storage: StorageMap<Address, bool> = storage.operator_approval.get(owner);
        operator_storage.get(operator)
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

    /// Gives operator approval to the to address to transfer
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    /// - The operator address provided is the 0 address
    /// - The address has already been approved
    /// - The sender is not the owner
    fn set_approval_for_all(owner: Address, operator: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(operator.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);

        let mut operator_storage: StorageMap<Address, bool> = storage.operator_approval.get(owner);
        require(!operator_storage.get(owner), Error::AddressAlreadyGivenApproval);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            require(owner != address, Error::SenderNotOwner);
  
            operator_storage.insert(operator, true); 
            storage.operator_approval.insert(owner, operator_storage);
        } else {
            revert(0);
        };

        true
    }

    /// Transfers ownership from one address to another
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not be initalized
    /// - The to address provided is the 0 address
    /// - The sender is not the owner
    /// - The sender is not approved
    /// - The sender is not an operator for the owner
    fn transfer_from(from: Address, to: Address, token_id: b256) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(to.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        
        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            let mut meta_data: MetaData = storage.meta_data.get(token_id);
            let operator_storage = storage.operator_approval.get(from);

            require(address == meta_data.owner 
                || address == meta_data.approved 
                || operator_storage.get(address)
                , Error::SenderNotOwnerOrApproved);

            meta_data.owner = to;
            meta_data.approved = ~Address::from(NATIVE_ASSET_ID);
            storage.meta_data.insert(token_id, meta_data);

            let mut balance = storage.balances.get(from);
            storage.balances.insert(from, balance - 1);
        } else {
            revert(0);
        };

        true
    }
}
