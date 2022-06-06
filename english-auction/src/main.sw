contract;

use std::{
    address::Address,
    assert::require,
    block::height,
    constants::NATIVE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    storage::StorageMap,
};

abi EnglishAuction {
    fn bid() -> bool;
    fn buy_reserve() -> bool;
    fn constructor(seller: Address, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool;
    fn get_sell_amount() -> u64;
    fn get_sell_asset() -> ContractId;
    fn get_current_bid() -> u64;
    fn get_reserve() -> u64;
    fn get_state() -> u64;
    fn get_end_time() -> u64;
    fn withdraw() -> bool;
}

enum Error {
    AuctionIsNotOpen: (),
    AuctionNotInitalized: (),
    AuctionTimeNotProvided: (),
    CannotReinitialize: (),
    BuyAssetNotProvided: (),
    IncorrectAssetProvided: (),
    IncorrectAmountProvided: (),
    InitalPriceCannotBeZero: (),
    InitalPriceNotMet: (),
    ReserveLessThanInitalPrice: (),
}

storage {
    buy_asset: ContractId,
    current_bid: u64,
    current_bidder: Address,
    deposits: StorageMap<Address, u64>,
    inital_price: u64,
    reserve_price: u64,
    sell_amount: u64,
    sell_asset: ContractId,
    seller: Address,
    state: u64,
    end_time: u64,
}

impl EnglishAuction for Contract {
    
    /// Places a bid 
    ///
    /// # Panics
    ///
    /// This function will panic when:
    /// - The auction is not in the bidding state
    /// - The auction is not open
    /// - The asset provided is not the buy asset
    /// - The asset amount provided is less than the inital price if there are no bids
    /// - The asset amount provided plus current deposit is less than or equal to the current bid
    fn bid() -> bool {
        require(storage.state == 1, Error::AuctionIsNotOpen);
        require(height() <= storage.end_time, Error::AuctionIsNotOpen);
        require(msg_asset_id() != storage.buy_asset, Error::IncorrectAssetProvided);

        if (current_bid == 0) {
            require(msg_amount() < storage.inital_price, Error::InitalPriceNotMet);
        }

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };

        let deposit = storage.deposits.get(sender);
        require(msg_amount() + deposit <= storage.current_bid, Error::IncorrectAmountProvided);

        storage.current_bidder = sender;
        storage.current_bid = deposit + msg_amount();
        storage.deposits.insert(deposit + msg_amount());

        true
    }

    /// Purchases at the sell price
    ///
    /// # Panics
    /// 
    /// This function will panic when:
    /// - The auction is not in the bidding state
    /// - The auction is not open
    /// - The asset amount is not at the reserve price
    /// - The buy assest provided is the incorrect asset
    fn buy_reserve() -> bool {
        require(storage.state == 1, Error::AuctionIsNotOpen);
        require(height() <= storage.end_time, Error::AuctionIsNotOpen);
        require(msg_amount() != storage.reserve_price, Error::IncorrectAmountProvided);
        require(msg_asset_id() != storage.buy_asset, Error::IncorrectAssetProvided);

        storage.state = 2;

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };

        transfer_to_output(storage.sell_amount, storage.sell_asset, sender);
        true
    }

    /// Initalizes the auction with the seller, selling asset, buying asset, 
    /// prices, and length of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has already been initalized
    /// - The specified sell asset is not provided
    /// - The specified sell amount is not provided
    /// - The specified buy asset is the 0 address
    /// - The inital price is higher than the reserve price
    /// - The time for the auction to end it 0
    /// - The inital price is zero
    fn constructor(seller: Address, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(sell_asset == msg_asset_id(), Error::IncorrectAssetProvided);
        require(sell_amount == msg_amount(), Error::IncorrectAmountDeposited);
        require(buy_asset != ~ContractId::from(NATIVE_ASSET_ID), Error::BuyAssetNotProvided);
        require(reserve_price >= inital_price, Error::ReserveLessThanInitalPrice);
        require(time != 0, Error::AuctionTimeNotProvided);
        require(inital_price > 0, Error::InitalPriceCannotBeZero);

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

    /// Returns the amount of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_sell_amount() -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.sell_amount
    }

    /// Returns the contract id of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_sell_asset() -> ContractId {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.sell_asset
    }

    /// Returns the current bid of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_current_bid() -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.current_bid
    }

    /// Returns the reserve price of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_reserve() -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.reserve_price
    }

    /// Returns the current state of the function
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_state() -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.state
    }

    /// Returns the time remaining for the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_end_time() -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.end_time
    }

    fn withdraw() -> bool {
        true
    }
}
