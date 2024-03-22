library;

use ::data_structures::state::State;

pub struct Auction {
    /// The asset which will be accepted in return for the selling asset.
    bid_asset: AssetId,
    /// The block at which the auction's bidding period should end.
    end_block: u32,
    /// The current highest bid.
    highest_bid: u64,
    /// The current highest bidder of the auction.
    highest_bidder: Option<Identity>,
    /// The starting price for the auction.
    initial_price: u64,
    /// The price at which the selling asset may be bought outright.
    reserve_price: Option<u64>,
    /// The asset that is being auctioned off.
    sell_asset: AssetId,
    /// The amount of the asset that is being auctioned off.
    sell_asset_amount: u64,
    /// The seller of the auction.
    seller: Identity,
    /// The state of the auction describing if it is open or closed.
    state: State,
}

impl Auction {
    /// Creates a new auction.
    ///
    /// # Arguments
    ///
    /// * `bid_asset`: [AssetId] - The asset which will be accepted in return for the selling asset.
    /// * `end_block`: [u32] - The block at which the auction's bidding period should end.
    /// * `initial_price`: [u64] - The starting price for the auction.
    /// * `reserve_price`: [Option<u64>] - The price at which the selling asset may be bought outright.
    /// * `sell_asset`: [AssetId] - The asset that is being auctioned off.
    /// * `sell_asset_amount`: [u64] - The amount of the asset that is being auctioned off.
    /// * `seller`: [Identity] - The seller of the auction.
    pub fn new(
        bid_asset: AssetId,
        end_block: u32,
        initial_price: u64,
        reserve_price: Option<u64>,
        sell_asset: AssetId,
        sell_asset_amount: u64,
        seller: Identity,
    ) -> Self {
        Auction {
            bid_asset,
            end_block,
            highest_bid: 0,
            highest_bidder: Option::None,
            initial_price,
            reserve_price,
            sell_asset,
            sell_asset_amount,
            seller,
            state: State::Open,
        }
    }
}
