library events;

dep data_structures;

use data_structures::Auction;

pub struct CancelledAuctionEvent {
    id: u64,
}

pub struct ChangedAsset {
    id: u64,
    new_asset: ContractId,
}

pub struct CreatedAuctionEvent {
    auction: Auction,
    id: u64,
}

pub struct WinningBidEvent {
    id: u64,
    winner: Identity,
}
