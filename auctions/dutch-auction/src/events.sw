library events;

dep data_structures;

use data_structures::Auction;
use std::identity::Identity;

pub struct AuctionCancelledEvent {
    id: u64,
}

pub struct CreatedAuctionEvent {
    auction: Auction,
    id: u64,
}

pub struct WinningBidEvent {
    winner: Identity,
    id: u64,
}
