library data_structures;

use std::{contract_id::ContractId, identity::Identity};

pub struct Auction {
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
    beneficiary: Identity,
    /// Whether the auction has ended
    ended: bool,
}
