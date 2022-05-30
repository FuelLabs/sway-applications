contract;

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, Sender, msg_sender},
    constants::NATIVE_ASSET_ID,
    result::*,
    revert::revert,
};

enum Error {
    
}

struct MetaData {

}

storage {
    metaData: MetaData
}

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

    fn constructor(owner: Address, access_control: bool, token_count: u64) -> bool {
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
