library events;

dep data_structures;

use data_structures::{Asset, Auction};
use std::identity::Identity;

pub struct AuctionStartEvent {
    auction: Auction,
    auction_id: u64,
}

pub struct BidEvent {
    asset: Asset,
    auction_id: u64,
    identity: Identity,
}

pub struct WithdrawEvent {
    asset: Asset,
    auction_id: u64,
    identity: Identity,
}
