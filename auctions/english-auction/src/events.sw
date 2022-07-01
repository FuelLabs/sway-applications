library events;

dep data_structures;

use data_structures::{Asset, Auction};
use std::identity::Identity;

pub struct CancelAuctionEvent {
    auction_id: u64,
}

pub struct CreateAuctionEvent {
    auction_id: u64,
}

pub struct BidEvent {
    amount: u64,
    auction_id: u64,
    identity: Identity,
}

pub struct WithdrawEvent {
    amount: u64,
    auction_id: u64,
    identity: Identity,
}
