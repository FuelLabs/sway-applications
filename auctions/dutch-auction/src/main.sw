contract;

dep abi;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{msg_sender, AuthError},
    context::{msg_amount, call_frames::msg_asset_id},
    contract_id::ContractId,
    identity::Identity,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::transfer_to_output,
};

use abi::DutchAuction;

struct Auction {
    /// Price at the very start, usually higher than any expected price of sale
    opening_price: u64,
    /// The Price that the auction will eventually reach if no bids are recieved. Can also be used as the reserve price
    reserve_price: u64,
    /// Point in time when bids can be placed and when the price will start to decrease
    start_time: u64,
    /// Only used for calculation of the price, users can still bid past this time for reserve_price unless it's ended by the admin
    end_time: u64,
    /// The asset the bidding will occur in
    asset_id: ContractId,
    /// The beneficiary of the proceeds of the auction
    beneficiary: Address,
    /// Whether the auction has ended
    ended: bool,
}


storage {
    /// Whether or not the constructor function has been called yet
    initialized: bool,
    /// Mapping an auction_id to its respective auction, allowing for multiple auctions to happen simultaneously
    auctions: StorageMap<u64, Auction>,
    /// The Admin Address
    admin: Address,
    /// Tracking how many auctions have been made till now
    auction_count: u64
}

enum Error {
    ContractNotYetInitialized: (),
    CannotReinitialize: (),
    SenderNotAdmin: (),
    AuctionInProgress: (),
    AuctionAlreadyEnded: (),
    BidTooLow: (),
    WrongAssetSent: (),
    EndPriceCannotBeLargerThanStartPrice: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    AuctionCannotEndBeforeItStarts: (),
    AuctionNotYetStarted: (),
    InvalidAuctionID: (),
}

impl DutchAuction for Contract {
    fn constructor(admin: Address) {
        require(!storage.initialized, Error::CannotReinitialize);
        storage.admin = admin;
        storage.initialized = true;
    }

    fn price(auction_id: u64) -> u64 {
        /// If the given auction id is higher than the auction count, its an invalid auction_id
        require(auction_id <= storage.auction_count, Error::InvalidAuctionID);

        calculate_price(auction_id)
    }

    fn bid(auction_id: u64) {
        /// In a Dutch auction the first bid wins
        require(storage.initialized, Error::ContractNotYetInitialized);

        /// If the given auction id is higher than the auction count, its an invalid auction_id
        require(auction_id <= storage.auction_count, Error::InvalidAuctionID);

        let mut auction = storage.auctions.get(auction_id);

        /// Cannot bid before auction starts
        require(height() >= auction.start_time, Error::AuctionNotYetStarted);

        /// Checks for correct asset_id being sent and high enough amount being sent
        require(msg_asset_id() == auction.asset_id, Error::WrongAssetSent);
        require(calculate_price(auction_id) <= msg_amount(), Error::BidTooLow);
        
        /// If ended == true, someone already bid or the admin prematurely ended the auction
        require(!auction.ended, Error::AuctionAlreadyEnded);

        /// Disallows furthur bids
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);

        /// If someone sends more than the current price, refund the extra amount 
        if msg_amount() > calculate_price(auction_id) {
            let return_amount = msg_amount() - calculate_price(auction_id);
            transfer_to_output(return_amount, auction.asset_id, get_sender());
        }

        /// Logic on win put into the win function. Using a function here so that its easier to modify the logic 
        /// post-auction-win and so this contract template can be reused for multiple things
        win(auction_id);
    }

    fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Address, asset: ContractId) -> u64 {
        require(storage.initialized, Error::ContractNotYetInitialized);
        require(storage.admin == get_sender(), Error::SenderNotAdmin);
        require(reserve_price <= opening_price, Error::EndPriceCannotBeLargerThanStartPrice);
        require(height() < end_time, Error::AuctionCannotEndInThePast);
        require(height() <= start_time, Error::AuctionCannotStartInThePast);
        require(start_time < end_time, Error::AuctionCannotEndBeforeItStarts);

        storage.auction_count = storage.auction_count + 1;
        let current_auction_id = storage.auction_count;

        let mut auction = storage.auctions.get(current_auction_id);
        auction.opening_price = opening_price;
        auction.reserve_price = reserve_price;
        auction.start_time = start_time;
        auction.end_time = end_time;
        auction.beneficiary = beneficiary;
        auction.asset_id = asset;
        
        storage.auctions.insert(current_auction_id, auction);

        /// Returns the auction being setup's auction_id
        current_auction_id
    }

    fn end_auction(auction_id: u64) {
        require(storage.initialized, Error::ContractNotYetInitialized);

        /// If the given auction id is higher than the auction count, its an invalid auction_id
        require(auction_id <= storage.auction_count, Error::InvalidAuctionID);

        /// Only the admin can end the auction (prematurely)
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        let mut auction = storage.auctions.get(auction_id);
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);
    }

}

fn win(auction_id: u64) {
    ///  Do stuff on a winning bid, this function is called whenever a winning bid is recieved.

    /// Add whatever logic you may want to execute on a win

    //Currently just sends the bid amount to the beneficiary
    let auction = storage.auctions.get(auction_id);
    transfer_to_output(calculate_price(auction_id), auction.asset_id, auction.beneficiary);
}

fn calculate_price(auction_id: u64) -> u64 {
    let auction = storage.auctions.get(auction_id);
    
    /// How much the price will go down by, throughout the auction
    let price_delta = auction.opening_price - auction.reserve_price;
    /// How long the auction will last
    let auction_duration = auction.end_time - auction.start_time;
    /// This is the amount the price will reduce by per block
    let price_shift = price_delta / auction_duration;

    /// Tells us how far we are into the auction (out of the auction_duration)
    let now = height() - auction.start_time; 

    /// Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let now = if now > auction_duration { auction_duration } else { now };

    /// price_shift * now tells us how much the price has reduced by now
    return auction.opening_price - (price_shift * now);
}

/// Helper function to avoid having to repeat this code
fn get_sender() -> Address {
    let a: Result<Identity, AuthError> = msg_sender();
    let b = a.unwrap();

    let addy = match b {
        Identity::Address(addr) => addr,
        _ => revert(0),
    };
    addy
}
