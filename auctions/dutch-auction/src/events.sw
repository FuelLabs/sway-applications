library events;

dep data_structures;

use data_structures::Auction;
use std::identity::Identity;

pub struct CancelledAuctionEvent {
    id: u64,
}

pub struct CreatedAuctionEvent {
    auction: Auction,
    id: u64,
}

pub struct WinningBidEvent {
    id: u64,
    winner: Identity,
}
