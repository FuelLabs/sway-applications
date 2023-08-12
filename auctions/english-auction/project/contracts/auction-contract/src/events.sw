library;

use ::data_structures::{auction::Auction, auction_asset::AuctionAsset};

/// Event for when an auction is cancelled.
pub struct CancelAuctionEvent {
    /// The auction id of the auction that was cancelled.
    auction_id: u64,
}

/// Event for when an auction is created.
pub struct CreateAuctionEvent {
    /// The auction id of the auction that was created.
    auction_id: u64,
    /// The asset in which bids will be recieved.
    bid_asset: AuctionAsset,
    /// The asset to be sold.
    sell_asset: AuctionAsset,
}

/// Event for when a bid is placed.
pub struct BidEvent {
    /// The amount of the bid.
    amount: u64,
    /// The auction id of the auction that was bid on.
    auction_id: u64,
    /// The bidder.
    user: Identity,
}

/// Event for when assets are withdrawn.
pub struct WithdrawEvent {
    /// The asset that was withdrawn.
    asset: AuctionAsset,
    /// The auction id of the auction that was withdrawn from.
    auction_id: u64,
    /// The user that withdrew the asset.
    user: Identity,
}
