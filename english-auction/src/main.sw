contract;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

abi EnglishAuction {
    fn bid() -> bool;
    fn buy_reserve() -> bool;
    fn constructor(seller: Identity, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool;
    fn get_balance(identity: Identity) -> u64;
    fn get_current_bid() -> u64;
    fn get_end_time() -> u64;
    fn get_highest_bidder() -> Option<Identity>;
    fn get_sell_amount() -> u64;
    fn get_sell_asset() -> ContractId;
    fn get_reserve() -> u64;
    fn get_state() -> u64;
    fn withdraw() -> bool;
}

enum Error {
    AuctionIsNotClosed: (),
    AuctionIsNotOpen: (),
    AuctionNotInitalized: (),
    AuctionTimeNotProvided: (),
    CannotReinitialize: (),
    BuyAssetNotProvided: (),
    IncorrectAssetProvided: (),
    IncorrectAmountProvided: (),
    InitalPriceCannotBeZero: (),
    InitalPriceNotMet: (),
    NoReserveSet: (),
    ReserveLessThanInitalPrice: (),
    UserHasAlreadyWithdrawn: (),
}

storage {
    buy_asset: ContractId,
    current_bid: u64,
    current_bidder: Identity,
    deposits: StorageMap<Identity, u64>,
    inital_price: u64,
    buyer_withdrawn: bool,
    reserve_price: u64,
    sell_amount: u64,
    sell_asset: ContractId,
    seller: Identity,
    seller_withdawn: bool,
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

        if (storage.current_bid == 0) {
            require(msg_amount() < storage.inital_price, Error::InitalPriceNotMet);
        }

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        let balance = storage.deposits.get(sender);
        require(msg_amount() + balance <= storage.current_bid, Error::IncorrectAmountProvided);

        storage.current_bidder = sender;
        storage.current_bid = balance + msg_amount();
        storage.deposits.insert(sender, balance + msg_amount());

        true
    }

    /// Purchases at the sell price
    ///
    /// # Panics
    /// 
    /// This function will panic when:
    /// - The auction is not in the bidding state
    /// - The auction is not open
    /// - There is no reserve price set
    /// - The asset amount is not at the reserve price
    /// - The buy assest provided is the incorrect asset
    fn buy_reserve() -> bool {
        require(storage.state == 1, Error::AuctionIsNotOpen);
        require(height() <= storage.end_time, Error::AuctionIsNotOpen);
        require(storage.reserve_price != 0, Error::NoReserveSet);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        let balance = storage.deposits.get(sender);
        require(msg_amount() + balance != storage.reserve_price, Error::IncorrectAmountProvided);
        require(msg_asset_id() != storage.buy_asset, Error::IncorrectAssetProvided);

        storage.state = 2;

        storage.current_bidder = sender;
        storage.current_bid = msg_amount() + balance;
        storage.buyer_withdrawn = true;

        match sender {
            Identity::Address(sender) => {
                transfer_to_output(storage.sell_amount, storage.sell_asset, sender);    
            },
            Identity::ContractId(sender) => {
                force_transfer_to_contract(storage.sell_amount, storage.sell_asset, sender);
            },
        };
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
    /// - The inital price is higher than the reserve price if a reserve price is set
    /// - The time for the auction to end it 0
    fn constructor(seller: Identity, sell_asset: ContractId, sell_amount: u64, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(sell_asset == msg_asset_id(), Error::IncorrectAssetProvided);
        require(sell_amount == msg_amount(), Error::IncorrectAmountProvided);
        require(buy_asset != ~ContractId::from(NATIVE_ASSET_ID), Error::BuyAssetNotProvided);
        require((reserve_price >= inital_price && reserve_price != 0) || reserve_price == 0, Error::ReserveLessThanInitalPrice);
        require(time != 0, Error::AuctionTimeNotProvided);

        storage.buy_asset = buy_asset;
        storage.buyer_withdrawn = false;
        storage.end_time = time + height();
        storage.inital_price = inital_price;
        storage.reserve_price = reserve_price;
        storage.sell_amount = sell_amount;
        storage.sell_asset = sell_asset;
        storage.seller = seller;
        storage.state = 1;
        storage.seller_withdawn = false;

        true
    }

    /// Returns the balance of the Address's buy asset deposits
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_balance(identity: Identity) -> u64 {
        require(storage.state != 0, Error::AuctionNotInitalized);
        storage.deposits.get(identity)
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

    /// Returns the current bidder of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction has not yet been initalized
    fn get_highest_bidder() -> Option<Identity> {
        require(storage.state != 0, Error::AuctionNotInitalized);
        Option::Some(storage.current_bidder)
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

    /// Withdraws after the end of the auction
    ///
    /// # Panics
    /// 
    /// The function will panic when:
    /// - The auction time is not over
    /// - The auction state is not over
    /// - The buyer is the sender and already withdrew
    /// - The seller is the sender and already withdrew
    /// - The sender is not the buyer or seller and has nothing to withdraw
    fn withdraw() -> bool {
        require(storage.state == 2 || height() >= storage.end_time, Error::AuctionIsNotClosed);

        // If time has run out set the contract state to 2
        if (height() >= storage.end_time && storage.state == 1)
        {
            storage.state = 2;
        }

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        let current_bidder = storage.current_bidder;
        let seller = storage.seller;

        match sender {
            // The buyer is withdrawing
            current_bidder => {
                require(!storage.buyer_withdrawn, Error::UserHasAlreadyWithdrawn);
                storage.buyer_withdrawn = true;

                match sender {
                    Identity::Address(sender) => {
                        transfer_to_output(storage.sell_amount, storage.sell_asset, sender);    
                    },
                    Identity::ContractId(sender) => {
                        force_transfer_to_contract(storage.sell_amount, storage.sell_asset, sender);
                    },
                };
            },
            // The seller is withdrawing
            seller => {
                require(!storage.seller_withdawn, Error::UserHasAlreadyWithdrawn);
                storage.seller_withdawn = true;

                // No one placed a bid
                if (storage.current_bid == 0) {
                    match sender {
                        Identity::Address(sender) => {
                            transfer_to_output(storage.sell_amount, storage.sell_asset, sender);    
                        },
                        Identity::ContractId(sender) => {
                            force_transfer_to_contract(storage.sell_amount, storage.sell_asset, sender);
                        },
                    };
                } else { 
                    match sender {
                        Identity::Address(sender) => {
                            transfer_to_output(storage.current_bid, storage.buy_asset, sender);    
                        },
                        Identity::ContractId(sender) => {
                            force_transfer_to_contract(storage.current_bid, storage.buy_asset, sender);
                        },
                    };
                }
            },
            // Anyone with a failed bid is withdrawing
            _ => {
                let deposit_amount = storage.deposits.get(sender);
                require(deposit_amount > 0, Error::UserHasAlreadyWithdrawn);

                storage.deposits.insert(sender, 0);

                match sender {
                    Identity::Address(sender) => {
                        transfer_to_output(deposit_amount, storage.buy_asset, sender);    
                    },
                    Identity::ContractId(sender) => {
                        force_transfer_to_contract(deposit_amount, storage.buy_asset, sender);
                    },
                };
            },
        }
        true
    }
}
