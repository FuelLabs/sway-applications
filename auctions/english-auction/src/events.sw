library events;

dep data_structures;

use data_structures::{Asset, Auction};

pub struct CancelAuctionEvent {
    auction_id: u64,
}

pub struct CreateAuctionEvent {
    auction_id: u64,
    bid_asset: Asset,
    sell_asset: Asset,
}

pub struct BidEvent {
    amount: u64,
    auction_id: u64,
    identity: Identity,
}

pub struct WithdrawEvent {
    asset: Asset,
    auction_id: u64,
    identity: Identity,
}
