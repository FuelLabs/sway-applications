contract;

use std::{
    address::Address,
    constants::NATIVE_ASSET_ID,
    contract_id::ContractId,
};

abi EnglishAuction {
    fn bid() -> bool;
    fn buy_reserve() -> bool;
    fn constructor(seller: Address, sell_asset: ContractId, buy_asset: ContractId, inital_price: u64, reserve_price: u64, end_time: u64) -> bool;
    fn get_sell_asset() -> ContractId;
    fn get_current_bid() -> u64;
    fn get_reserve() -> u64;
    fn get_state() -> u64;
    fn get_time_remaining() -> u64;
    fn withdraw() -> bool;
}

impl EnglishAuction for Contract {
    fn bid() -> bool {
        true
    }

    fn buy_reserve() -> bool {
        true
    }

    fn constructor(seller: Address, sell_asset: ContractId, buy_asset: ContractId, inital_price: u64, reserve_price: u64, end_time: u64) -> bool {
        true
    }

    fn get_sell_asset() -> ContractId {
        ~ContractId::from(NATIVE_ASSET_ID)
    }

    fn get_current_bid() -> u64 {
        0
    }

    fn get_reserve() -> u64 {
        0
    }

    fn get_state() -> u64 {
        0
    }

    fn get_time_remaining() -> u64 {
        0
    }

    fn withdraw() -> bool {
        true
    }
}
