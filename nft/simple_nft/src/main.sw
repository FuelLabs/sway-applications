contract;

use std::{
    address::Address,
};
use nft_lib::*;

abi SimpleNFT {
    fn burn() -> bool;
    fn mint(address: Address) -> bool;
    fn set_approval(address: Address) -> bool;
    fn get_approval() -> Address;
    fn transfer(from: Address, to: Address) -> bool;
    fn owner() -> Address;
}

impl SimpleNFT for Contract {

    fn burn() -> bool {
        burn()
    }
    
    fn mint(address: Address) -> bool {
        mint_to_address(address)
    }

    fn set_approval(address: Address) -> bool {
        set_transfer_approval(address)
    }
    
    fn get_approval() -> Address {
        get_transfer_approval()
    }

    fn transfer(from: Address, to: Address) -> bool {
        transfer_to_output(from, to)
    }

    fn owner() -> Address {
        get_owner()
    }
}