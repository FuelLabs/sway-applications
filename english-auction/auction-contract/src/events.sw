library;

use ::data_structures::auction::Auction;

/// Event for when an auction is cancelled.
pub struct CancelAuctionEvent {
    /// The auction id of the auction that was cancelled.
    pub auction_id: u64,
}

/// Event for when an auction is created.
pub struct CreateAuctionEvent {
    /// The auction id of the auction that was created.
    pub auction_id: u64,
    /// The asset in which bids will be received.
    pub bid_asset: AssetId,
    /// The asset to be sold.
    pub sell_asset: AssetId,
    /// The amount of the asset being sold.
    pub sell_asset_amount: u64,
}

/// Event for when a bid is placed.
pub struct BidEvent {
    /// The amount of the bid.
    pub amount: u64,
    /// The auction id of the auction that was bid on.
    pub auction_id: u64,
    /// The bidder.
    pub user: Identity,
}

/// Event for when assets are withdrawn.
pub struct WithdrawEvent {
    /// The asset that was withdrawn.
    pub asset: AssetId,
    /// The amount of the asset that is withdrawn.
    pub asset_amount: u64,
    /// The auction id of the auction that was withdrawn from.
    pub auction_id: u64,
    /// The user that withdrew the asset.
    pub user: Identity,
}
