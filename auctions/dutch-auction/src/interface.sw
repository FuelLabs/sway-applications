library interface;

dep data_structures;

use data_structures::Auction;

abi DutchAuction {
    /// Returns the number of active auctions of the author
    ///
    /// # Argumets
    /// 
    /// * `author` - The author of which to retrieve the number of active auctions of
    #[storage(read)]
    fn active_auctions_of_author(author: Identity) -> u64;
    /// Returns the auction_id of the active auction of the author at the given index
    ///
    /// # Arguments
    /// 
    /// * `author` - The author of which to retrieve the active auction of
    /// * `index` - The index to retrieve
    ///
    /// # Reverts
    ///
    /// * When the index is greater than the number of active auctions of the author
    #[storage(read)]
    fn active_auction_of_author(author: Identity, index: u64) -> u64;
    
    /// Returns the auction data for the specified auction ID
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id of the auction of which to fetch the data of 
    /// 
    /// # Reverts
    ///
    /// * When the auction_id is 0 or greater than storage.auction_count
    #[storage(read)]
    fn auction(auction_id: u64) -> Auction;
    /// Returns the number of auctions created by author
    ///
    /// # Arguments
    ///
    /// * `author` - The Identity of which to check the number of auctions created
    #[storage(read)]
    fn auctions_of_author(author: Identity) -> u64;
    /// Returns the number of auctions some bidder has won
    ///
    /// # Arguments
    ///
    /// * `bidder` - The Identity of which to check the number of auctions won
    #[storage(read)]
    fn auctions_won(bidder: Identity) -> u64;
    /// Returns the auction_id of the auction of the author at the given index
    ///
    /// # Arguments
    /// 
    /// * `author` - The author of which to retrieve the auction of
    /// * `index` - The index to retrieve
    ///
    /// # Reverts
    ///
    /// * When the index is greater than the number of auctions of the author
    #[storage(read)]
    fn auction_of_author(author: Identity, index: u64) -> u64;
    /// Returns the auction_id of the won auction of the bidder at the given index
    ///
    /// # Arguments
    /// 
    /// * `bidder` - The bidder of which to retrieve the auction of
    /// * `index` - The index to retrieve
    ///
    /// # Reverts
    ///
    /// * When the index is greater than the number of auctions won by the bidder
    #[storage(read)]
    fn auction_won(bidder: Identity, index: u64) -> u64;
    /// Bids on the specified auction
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id of the auction on which to
    ///
    /// # Reverts
    ///
    /// * When auction_id is 0 or greater than storage.auction_count
    /// * When the auction has already ended
    /// * When the current block height is lower than start_time, or higher than end_time
    /// * When the Incorrect asset is sent to the auction
    /// * When the bid is less than the current price
    #[storage(read, write)]
    fn bid(auction_id: u64);
    
    /// Cancels an auction preventing any bids from being placed
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id of the auction to cancel
    ///
    /// # Reverts
    ///
    /// * When the auction_id is 0 or greater than storage.auction_count
    /// * When the msg_sender is not the author of the auction
    /// * When the auction has already ended
    #[storage(read, write)]
    fn cancel_auction(auction_id: u64);
    
    /// Changes the asset an auction accepts for a bid
    ///
    /// # Arguments
    /// 
    /// * `new_asset` - The asset which will henceforth be used for the auction
    /// * `auction_id` - The id of the auction of which to change the asset of
    ///
    /// # Reverts
    ///
    /// * When the auction_id is 0 or greater than storage.auction_count
    /// * When the msg_sender is not the author of the auction
    /// * When the auction has already ended
    #[storage(read, write)]
    fn change_asset(new_asset: ContractId, auction_id: u64);
    
    /// Changes the beneficiary of the given auction
    ///
    /// # Arguments
    ///
    /// * `new_beneficiary` - The Identity which will henceforth recieve the proceeds of the auction
    /// * `auction_id` - The id of the auction of which to change the beneficiary of
    ///
    /// # Reverts
    ///
    /// * When the auction_id is 0 or greater than storage.auction_count
    /// * When the msg_sender is not the author of the auction
    /// * When the auction has already ended
    #[storage(read, write)]
    fn change_beneficiary(new_beneficiary: Identity, auction_id: u64);
    /// Creates a new auction
    ///
    /// # Arguments
    ///
    /// * `opening_price` - The price at the start of the auction
    /// * `reserve_price` - The minimum bid required to win the auction
    /// * `start_time` - The block height at which the auction will start
    /// * `end_time` - The block height at which the required bid amount will reach the reserve price and at which point the auction will end
    /// * `beneficiary` - The Identity to which the proceeds of the auction will be sent
    ///
    /// # Reverts
    ///
    /// * When the reserve_price is greater than opening_price
    /// * When the block height is greater than end_time or start_time
    /// * When the start_time is greater than end_time
    #[storage(read, write)]
    fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Identity, asset: ContractId);
    /// Returns the current price for the auction corresponding to the auction_id
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The id of the auction of which to check the price
    ///
    /// # Reverts
    ///
    /// * When auction_id is 0 or higher than storage.auction_count
    #[storage(read)]
    fn price(auction_id: u64) -> u64;
}
