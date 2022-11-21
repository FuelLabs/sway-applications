library data_structures;

pub struct Auction {
    /// The asset the auction accepts for placing bids
    asset_id: ContractId,
    /// The identity that created the auction
    author: Identity,
    /// The beneficiary of the proceeds of the auction
    beneficiary: Identity,
    /// Whether the auction has ended
    ended: bool,
    /// When the auction ends
    end_time: u64,
    /// Price at the very start, usually higher than any expected price of sale
    opening_price: u64,
    /// The Price that the auction will eventually reach if no bids are recieved. Can also be used as the reserve price
    reserve_price: u64,
    /// Point in time when bids can be placed and when the price will start to decrease
    start_time: u64,
    /// The Identity which won the auction
    winner: Option<Identity>,
}
