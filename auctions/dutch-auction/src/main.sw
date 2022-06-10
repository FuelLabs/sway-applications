contract;

dep abi;
dep errors;
dep data_structures;

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
use errors::Error;
use data_structures::Auction;

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
        let price = calculate_price(auction_id);

        /// Cannot bid before auction starts
        require(height() >= auction.start_time, Error::AuctionNotYetStarted);

        /// Checks for correct asset_id being sent and high enough amount being sent
        require(msg_asset_id() == auction.asset_id, Error::WrongAssetSent);
        require(price <= msg_amount(), Error::BidTooLow);
        
        /// If ended == true, someone already bid or the admin prematurely ended the auction
        require(!auction.ended, Error::AuctionAlreadyEnded);

        /// Disallows furthur bids
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);

        /// If someone sends more than the current price, refunds the extra amount 
        if msg_amount() > price {
            let return_amount = msg_amount() - price;
            transfer_to_output(return_amount, auction.asset_id, get_sender());
        }

        on_win(auction_id, price);
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

/// This function is called whenever a winning bid is recieved.
fn on_win(auction_id: u64, winning_amount: u64) {
    let auction = storage.auctions.get(auction_id);
    transfer_to_output(winning_amount, auction.asset_id, auction.beneficiary);
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
    let blocks_into_auction = height() - auction.start_time; 

    /// Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let blocks_into_auction = if blocks_into_auction > auction_duration { auction_duration } else { blocks_into_auction };

    /// price_shift * blocks_into_auction tells us how much the price has reduced by now
    auction.opening_price - (price_shift * blocks_into_auction)
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
