contract;

use std::{
    address::Address,
    assert::require,
    block::height,
    constants::NATIVE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
};

abi EnglishAuction {
    fn bid() -> bool;
    fn buy_reserve() -> bool;
    fn constructor(seller: Address, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool;
    fn get_sell_asset() -> ContractId;
    fn get_current_bid() -> u64;
    fn get_reserve() -> u64;
    fn get_state() -> u64;
    fn get_time_remaining() -> u64;
    fn withdraw() -> bool;
}

enum Error {
    AuctionTimeNotProvided: (),
    CannotReinitialize: (),
    BuyAssetNotProvided: (),
    ReserveLessThanInitalPrice: (),
    SellAssetNotDeposited: (),
    SellAmountNotDeposited: (),
}

storage {
    buy_asset: ContractId,
    inital_price: u64,
    reserve_price: u64,
    sell_amount: u64,
    sell_asset: ContractId,
    seller: Address,
    state: u64,
    end_time: u64,
}

impl EnglishAuction for Contract {
    fn bid() -> bool {
        true
    }

    fn buy_reserve() -> bool {
        true
    }

    /// Initalizes the auction with the seller, selling asset, buying asset, 
    /// prices, and length of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The contract has already been initalized
    /// - The specified sell asset is not provided
    /// - The specified sell amount is not provided
    /// - The specified buy asset is the 0 address
    /// - The inital price is higher than the reserve price
    /// - The time for the auction to end it 0
    fn constructor(seller: Address, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(sell_asset == msg_asset_id(), Error::SellAssetNotDeposited);
        require(sell_amount == msg_amount(), Error::SellAmountNotDeposited);
        require(buy_asset != ~ContractId::from(NATIVE_ASSET_ID), Error::BuyAssetNotProvided);
        require(reserve_price >= inital_price, Error::ReserveLessThanInitalPrice);
        require(time != 0, Error::AuctionTimeNotProvided);

        storage.buy_asset = buy_asset;
        storage.inital_price = inital_price;
        storage.reserve_price = reserve_price;
        storage.sell_amount = sell_amount;
        storage.sell_asset = sell_asset;
        storage.seller = seller;
        storage.state = 1;
        storage.end_time = time + height();

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
