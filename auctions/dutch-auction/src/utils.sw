library;

use std::{
    block::height,
    hash::sha256,
    storage::{
        get,
        store,
    },
    token::{
        force_transfer_to_contract,
        transfer_to_address,
    },
};

use ::data_structures::Auction;
use ::errors::UserError;

/// Calculates the current price of a given auction
pub fn calculate_price(auction: Auction) -> u64 {
    // How much the price will go down by, throughout the auction
    let price_delta = auction.opening_price - auction.reserve_price;
    // How long the auction will last
    let auction_duration = auction.end_time - auction.start_time;
    // This is the amount the price will reduce by per block
    let price_shift = price_delta / auction_duration;

    // Tells us how far we are into the auction (out of the auction_duration)
    let blocks_into_auction = height() - auction.start_time;

    // Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let blocks_into_auction = if blocks_into_auction > auction_duration {
        auction_duration
    } else {
        blocks_into_auction
    };

    // price_shift * blocks_into_auction tells us how much the price has reduced by now
    auction.opening_price - (price_shift * blocks_into_auction)
}

/// Validates an auction_id to make sure it corresponds to an auction
pub fn validate_id(id: u64, auction_count: u64) {
    // If the id is greater than the auction count then it's invalid
    require(id != 0, UserError::InvalidAuctionID);
    require(id <= auction_count, UserError::InvalidAuctionID);
}
