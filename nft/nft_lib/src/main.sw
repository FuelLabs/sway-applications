library nft_lib;

use std::{
    address::Address,
    assert::require,
    constants::NATIVE_ASSET_ID,
    chain::auth::{AuthError, Sender, msg_sender},
    result::*,
    revert::revert,
};

enum Error {
    AddressCannotBeZero: (),
    NFTNotInitalized: (),
    OwnerAlreadySet: (),
    OwnerCannotBeApproved: (),
    SenderNotOwner: (), 
}

// TODO: Add mappings when supported for token ids

struct Data {
    owner: Address,
    approved: Address,
}

storage {
    data: Data,
    state: u64,
}

/// Sets the inital owner for the NFT
///
/// # Panics
///
/// The function will panic when:
/// - The NFT has already been initalized
/// - The new owner is the 0 address
pub fn mint_to_address(new_owner: Address) -> bool {
    require(storage.state == 0, Error::OwnerAlreadySet);
    require(new_owner.value != NATIVE_ASSET_ID, Error::AddressCannotBeZero);

    storage.data = Data {
        owner: new_owner, approved: ~Address::from(NATIVE_ASSET_ID)
    };
    storage.state = 1;
    true
}

/// Burns the NFT
///
/// # Panics
///
/// The function will panic when:
/// - The sender is not the owner
/// - The NFT has not be initalized
pub fn burn() -> bool {
    require(storage.state == 1, Error::NFTNotInitalized);

    let sender: Result<Sender, AuthError> = msg_sender();
    if let Sender::Address(address) = sender.unwrap() { 
        require(address == storage.data.owner, Error::SenderNotOwner);

        storage.data.owner = ~Address::from(NATIVE_ASSET_ID);
        storage.state = 2;
    } else {
        revert(0);
    };

    true
}

/// Transfers the NFT from one address to another
/// 
/// # Panics
///
/// The function will panic when:
/// - The sender is not the owner or the sender isn't approved
pub fn transfer_to_output(owner: Address, to: Address) -> bool {
    require(storage.state == 1, Error::NFTNotInitalized);

    let sender: Result<Sender, AuthError> = msg_sender();
    if let Sender::Address(address) = sender.unwrap() {
        require(address == storage.data.owner || address == storage.data.approved, Error::SenderNotOwner);

        storage.data.owner = to;
        storage.data.approved = ~Address::from(NATIVE_ASSET_ID);
    } else {
        revert(0);
    };

    true
}

/// Returns the address of the owner
///
/// # Panics
// 
/// the function will panic when:
/// - The NFT has not been initalized
pub fn get_owner() -> Address {
    require(storage.state != 0, Error::NFTNotInitalized);
    storage.data.owner
}

/// Returns the address set for transfer approval
///
/// # Panics
// 
/// the function will panic when:
/// - The NFT has not been initalized
pub fn get_transfer_approval() -> Address {
    require(storage.state != 0, Error::NFTNotInitalized);
    storage.data.approved
}

/// Sets the address to approved for transfer
///
/// # Panics
///
/// The function will panic when:
/// - The sender is not the owner
/// - The owner is the to address
/// - The NFT has not been initalized
pub fn set_transfer_approval(to: Address) -> bool {
    require(storage.state == 1, Error::NFTNotInitalized);

    let sender: Result<Sender, AuthError> = msg_sender();
    if let Sender::Address(address) = sender.unwrap() {
        require(address == storage.data.owner, Error::SenderNotOwner);
        require(to != storage.data.owner, Error::OwnerCannotBeApproved);

        storage.data.approved = to;
    } else {
        revert(0);
    };

    true
}