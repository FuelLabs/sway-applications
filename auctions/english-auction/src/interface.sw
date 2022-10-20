library interface;

dep data_structures;

use data_structures::{Asset, Auction};

abi EnglishAuction {
    /// Returns the auction struct for the corresponding auction id.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id number of the auction.
    #[storage(read)]
    fn auction_info(auction_id: u64) -> Option<Auction>;

    /// Places a bid on the auction specified.
    ///
    /// An asset must be provided with a corresponding struct. A bid is only valid if it is greater
    /// than the last bid or greater than the initial_price. If the reserve price is met, the auction will end.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id number of the auction.
    /// * `bid_asset` - An asset that is either a `TokenAsset` struct or a `NFTAsset` struct.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the auction has closed.
    /// * When the bidding period for the auction has passed.
    /// * When the asset provided does not match the asset accepted for the auction.
    /// * When the bidder does not own the NFT that is being bid.
    /// * When the auction contract does not have permission to transfer the NFT to it's ownership.
    /// * When the amount provided the the struct do not match.
    /// * When the asset provided and struct do not match.
    /// * When the bidder is the auction's `seller`.
    /// * When the bid amount is less than the initial price.
    /// * When the total of previous bids plus this bid are not greater than the current bid amount.
    /// * When the total of previous bids plus this bid is greater than the reserve price.
    #[storage(read, write)]
    fn bid(auction_id: u64, bid_asset: Asset);

    /// Cancels the specified auction.
    ///
    /// Once the auction has been canceled users will be able to withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the auction is no longer open.
    /// * When the `sender` is not the `seller` of the auction.
    #[storage(read, write)]
    fn cancel(auction_id: u64);

    /// Starts an auction with a seller, selling asset, buying asset, prices, and duration of the
    /// auction.
    ///
    /// This function will return the the newly created auction's ID number which must be used to
    /// interact with the auction.
    ///
    /// # Arguments
    ///
    /// `bid_asset` - The asset the seller is willing to accept in return for the selling asset.
    /// `duration` - The length of time the auction should be open.
    /// `initial_price` - The starting price at which the auction should start.
    /// `reserve_price` - The price at which a buyer may purchase the `sell_asset` outright.
    /// `seller` - The seller for this auction.
    /// `sell_asset` - The struct that contains information about what is being auctioned off.
    ///
    /// # Reverts
    ///
    /// * When the `reserve_price` is less than `initial_price` if a `reserve_price` is set.
    /// * When the `duration` of the auction is set to zero.
    /// * When the inital price for tokens is set to zero.
    /// * When the transaction's token amount is not the amount specified in the `sell_asset` struct.
    /// * When the transaction's token is not the same as the specified in the `sell_asset` struct.
    /// * When setting the quantity of NFTs to accept is not one.
    /// * When the sender is not the owner of the NFT's provided.
    /// * When the auction contract is not approved to transfer the NFT's provided.
    #[storage(read, write)]
    fn create(bid_asset: Asset, duration: u64, inital_price: u64, reserve_price: Option<u64>, seller: Identity, sell_asset: Asset) -> u64;

    /// Returns the balance of the user's deposits for the specified auction.
    ///
    /// # Arguments
    ///
    /// * `identity` - The user which has deposited assets.
    /// * `auction_id` - The id number of the auction.
    #[storage(read)]
    fn deposit(auction_id: u64, identity: Identity) -> Option<Asset>;

    /// Allows users to withdraw their assets if the auction's bid period has ended, the reserve has
    /// been met, or has been canceled.
    ///
    /// If there is a winning bidder, the winning bidder will withdraw the selling asset, the failed
    /// bidders will withdraw their original deposit, and the seller will withdraw the winning
    /// bidder's deposit. If there is no winning bidder or the auction was canceled, the seller and
    /// bidder will all withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` provided does not map to an existing auction.
    /// * When the duration of the auction has not ended.
    /// * When the auction's `state` is still in the open bidding state.
    /// * When the sender has already withdrawn their deposit.
    #[storage(read, write)]
    fn withdraw(auction_id: u64);

    /// Returns the total auctions which have been started using this auction contract.
    #[storage(read)]
    fn total_auctions() -> u64;
}

abi NFT {
    fn approved(token_id: u64) -> Identity;
    fn is_approved_for_all(operator: Identity, owner: Identity) -> bool;
    fn owner_of(token_id: u64) -> Identity;
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}
