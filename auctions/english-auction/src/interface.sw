library interface;

dep data_structures;

use data_structures::{Asset, Auction};
use std::{contract_id::ContractId, identity::Identity, option::Option};

abi EnglishAuction {
    /// Returns the auction struct for the corresponding auction id.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the auction id does not map to an existing auction
    #[storage(read)]
    fn auction_info(auction_id: u64) -> Auction;

    /// Places a bid on the auction specified.
    ///
    /// A correctly structured `Asset` struct must be provided. A bid is only valid if it is
    /// greater than the last bid or greater than the initial_price. If the reserve price is met,
    ///  the auction will end.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    /// * `new_bid_asset` - An `Asset` enum that is either a `TokenAsset` struct or a `NFTAsset` struct.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the auction has closed.
    /// * When the bidding time for the auction has closed.
    /// * When the `new_bid_asset` `ContractId` provided does not match the auction's `bid_asset`
    ///   `ContractId`.
    /// * When the `new_bid_asset` amount provided does not match the transaction's `msg_amount`.
    /// * When the auction contract does not have permission to transfer the NFT to it's ownership.
    /// * When the bidder/sender is the auction's `seller`.
    /// * When the total of previous plus this bid is greater than the reserve price.
    /// * When the `new_bid_asset` amount provided is less than the initial price if there are no bids.
    /// * When the total of previous plus this bid amounts are not greater than the current bid
    ///   amount.
    #[storage(read, write)]
    fn bid(auction_id: u64, bid_asset: Asset);

    /// Cancels the specified auction.
    ///
    /// Once the auction has been canceled user will be able to withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the `sender` is not the `seller` of the auction.
    #[storage(read, write)]
    fn cancel(auction_id: u64);

    /// Starts an auction with a seller, selling asset, buying asset, prices, and length of the
    /// auction.
    ///
    /// This function will return the the newly created auction's ID number which must be used to
    /// interact with the auction.
    ///
    /// # Arguments
    ///
    /// `seller` - The `Identity` of the seller for this auction. This `Identity` will have the
    ///            ability to cancel and withdraw the originially provided assets.
    /// `sell_asset` - The `Asset` struct that contains information about what is being auctioned
    ///                off.
    /// `bid_asset` - The `Asset` struct that contains the `contract_id` of the asset the seller is
    ///               willing to accept in return for the `sell_asset`.
    /// `initial_price` - The starting price at which the auction should start.
    /// `reserve_price` - The price at which a buyer may purchase the `sell_asset` outright.
    /// `time` - The duration of the auction in number of blocks.
    ///
    /// # Reverts
    ///
    /// * When the `initial_price` is higher than the `reserve_price` if a `reserve_price` is set.
    /// * When the `time` or duration of the auction is set to zero.
    /// * When the transaction's token amount tranfered is not the amount specified in the
    ///   `sell_asset` struct.
    /// * When the transaction's token `contract_id` is not the same as the `contract_id` specified
    ///   in the `sell_asset` struct.
    /// * When the `sender` is not the owner of the NFT's provided in the `sell_asset` struct.
    /// * When the auction contract is not approved to transfer the NFT's provided in the
    ///   `sell_asset` struct.
    #[storage(read, write)]
    fn create(buy_asset: Asset, inital_price: u64, reserve_price: u64, seller: Identity, sell_asset: Asset, time: u64) -> u64;

    /// Returns the balance of the user's `bid_asset` deposits.
    ///
    /// # Arguments
    ///
    /// * `identity` - The `Identity` of the user which has deposited assets
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the idendity and auction id provided do not map to an existing auction
    #[storage(read)]
    fn deposit(auction_id: u64, identity: Identity) -> Asset;

    /// Allows users to withdraw their assets if the auction has gone over time, the reserve has
    /// been met, or been canceled.
    ///
    /// If there is a winning bidder, the winning bidder will withdraw the `sell_asset`, the failed
    /// bidders will withdraw their original deposit, and the seller will withdraw the winning
    /// bidder's deposit. If there is no winning bidder or the auction was canceled, the seller and
    /// bidder will withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` provided does not map to an existing auction.
    /// * When the duration of the auction has not ended.
    /// * When the auction's `state` is still in the open bidding state.
    /// * When the `sender` has already withdrawn their deposit.
    #[storage(read, write)]
    fn withdraw(auction_id: u64);

    /// Returns the total auctions which have been started using this auction contract.
    #[storage(read)]
    fn total_auctions() -> u64;
}

abi NFT {
    fn approved(token_id: u64) -> Option<Identity>;
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    fn owner_of(token_id: u64) -> Option<Identity>;
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}
