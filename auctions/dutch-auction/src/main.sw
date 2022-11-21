contract;

dep contract_abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use std::{block::height, call_frames::msg_asset_id, context::msg_amount, logging::log};

use contract_abi::DutchAuction;
use data_structures::Auction;
use errors::{SetupError, TimeError, UserError};
use events::{CancelledAuctionEvent, ChangedAsset, CreatedAuctionEvent, WinningBidEvent};
use utils::{calculate_price, eq_identity, sender_indentity, transfer_to_identity, validate_id};

storage {
    /// Mapping an auction_id to its respective auction, allowing for multiple auctions to happen simultaneously
    auctions: StorageMap<u64, Auction> = StorageMap {},
    /// Tracking how many auctions have been made till now
    auction_count: u64 = 0,
    /// Auction ids of the active auctions by author
    active_auctions_of_author: StorageMap<Identity, [u64; 1]> = StorageMap {},
    /// Auction ids of all auctions by author
    auctions_of_author: StorageMap<Identity, [u64; 1]> = StorageMap {},
    /// The auctions which any given bidder has won
    auctions_won: StorageMap<Identity, [u64; 1]> = StorageMap {},
}

impl DutchAuction for Contract {
    /// Returns the current price for the auction corresponding to the auction_id
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or higher than storage.auction_count
    #[storage(read)]
    fn price(auction_id: u64) -> u64 {
        validate_id(auction_id, storage.auction_count);
        calculate_price(storage.auctions.get(auction_id))
    }

    /// Bids on the specified auction
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. auction has already ended
    ///     3. current block height is lower than start_time, or higher than end_time
    ///     4. Incorrect asset is sent to the auction
    ///     5. The bid is less than the current price
    #[storage(read, write)]
    fn bid(auction_id: u64) {
        // In a Dutch auction the first bid wins
        validate_id(auction_id, storage.auction_count);

        let mut auction = storage.auctions.get(auction_id);

        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        // Cannot bid before auction starts
        require(auction.start_time <= height(), TimeError::AuctionNotYetStarted);
        // Cannot bid after auction ends
        require(height() <= auction.end_time, TimeError::AuctionAlreadyEnded);

        // Checks for correct asset_id being sent
        require(msg_asset_id() == auction.asset_id, UserError::WrongAssetSent);

        let price = calculate_price(auction);

        // Checks for high enough amount being sent
        require(price <= msg_amount(), UserError::BidTooLow);

        // Disallows furthur bids
        auction.ended = true;
        auction.winner = Option::Some(sender_indentity());
        storage.auctions.insert(auction_id, auction);

        // If someone sends more than the current price, refunds the extra amount
        if msg_amount() > price {
            let return_amount = msg_amount() - price;
            transfer_to_identity(return_amount, auction.asset_id, sender_indentity());
        }

        on_win(auction, price);

        // WARNING: This needs to be changed to a pop to a vec instead of just replacing the contents of the array
        storage.active_auctions_of_author.insert(auction.author, [0]);
        // WARNING: This needs to be changed to a push to a vec instead of just replacing the contents of the array
        storage.auctions_won.insert(sender_indentity(), [auction_id]);

        log(WinningBidEvent {
            id: auction_id,
            winner: sender_indentity(),
        });
    }

    /// Creates a new auction
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. reserve_price is greater than opening_price
    ///     2. block height is greater than end_time or start_time
    ///     3. start_time is greater than end_time
    #[storage(read, write)]
    fn create_auction(
        opening_price: u64,
        reserve_price: u64,
        start_time: u64,
        end_time: u64,
        beneficiary: Identity,
        asset: ContractId,
    ) {
        require(reserve_price <= opening_price, SetupError::EndPriceCannotBeLargerThanStartPrice);
        require(height() < end_time, SetupError::AuctionCannotEndInThePast);
        require(height() <= start_time, SetupError::AuctionCannotStartInThePast);
        require(start_time < end_time, SetupError::AuctionCannotEndBeforeItStarts);

        let auction = Auction {
            opening_price,
            reserve_price,
            start_time,
            end_time,
            beneficiary,
            asset_id: asset,
            author: sender_indentity(),
            ended: false,
            winner: Option::None,
        };

        storage.auction_count = storage.auction_count + 1;
        storage.auctions.insert(storage.auction_count, auction);

        // WARNING: This needs to be changed to a push to a vec instead of just replacing the contents of the array
        storage.active_auctions_of_author.insert(sender_indentity(), [storage.auction_count]);
        // WARNING: This needs to be changed to a push to a vec instead of just replacing the contents of the array
        storage.auctions_of_author.insert(sender_indentity(), [storage.auction_count]);

        log(CreatedAuctionEvent {
            auction,
            id: storage.auction_count,
        });
    }

    /// Cancels an auction preventing any bids from being placed
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the author of the auction
    ///     3. auction has already ended
    #[storage(read, write)]
    fn cancel_auction(auction_id: u64) {
        validate_id(auction_id, storage.auction_count);

        let mut auction = storage.auctions.get(auction_id);

        // Only the author can end the auction (prematurely)
        require(eq_identity(sender_indentity(), auction.author), UserError::SenderNotAuthor);
        // Cannot cancel an auction that has already ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.ended = true;
        storage.auctions.insert(auction_id, auction);
        // WARNING: This needs to be changed to a pop to a vec instead of just replacing the contents of the array
        storage.active_auctions_of_author.insert(sender_indentity(), [0]);

        log(CancelledAuctionEvent {
            id: auction_id,
        });
    }

    /// Returns the auction data for the specified auction ID
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    #[storage(read)]
    fn auction(auction_id: u64) -> Auction {
        validate_id(auction_id, storage.auction_count);
        storage.auctions.get(auction_id)
    }

    /// Changes the asset an auction accepts for a bid
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the author of the auction
    ///     3. auction has already ended
    #[storage(read, write)]
    fn change_asset(new_asset: ContractId, auction_id: u64) {
        validate_id(auction_id, storage.auction_count);
        let mut auction = storage.auctions.get(auction_id);

        // Only the author can change the bidding asset
        require(eq_identity(sender_indentity(), auction.author), UserError::SenderNotAuthor);
        // Cannot edit an auction that has ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.asset_id = new_asset;

        storage.auctions.insert(auction_id, auction);

        log(ChangedAsset {
            new_asset,
            id: auction_id,
        });
    }

    /// Changes the beneficiary of the given auction
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the author of the auction
    ///     3. auction has already ended
    #[storage(read, write)]
    fn change_beneficiary(new_beneficiary: Identity, auction_id: u64) {
        validate_id(auction_id, storage.auction_count);
        let mut auction = storage.auctions.get(auction_id);

        // Only the author can change the beneficiary
        require(eq_identity(sender_indentity(), auction.author), UserError::SenderNotAuthor);
        // Cannot edit an auction that has ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.beneficiary = new_beneficiary;

        storage.auctions.insert(auction_id, auction);
    }

    /// Returns the active auctions of the author
    #[storage(read)]
    fn active_auctions_of_author(author: Identity) -> [u64; 1] {
        storage.active_auctions_of_author.get(author)
    }

    /// Returns all the auctions created by author
    #[storage(read)]
    fn auctions_of_author(author: Identity) -> [u64; 1] {
        storage.auctions_of_author.get(author)
    }

    /// Returns what auctions some bidder has won
    #[storage(read)]
    fn auctions_won(bidder: Identity) -> [u64; 1] {
        storage.auctions_won.get(bidder)
    }
}

/// This function is called whenever a winning bid is recieved.
fn on_win(auction: Auction, winning_amount: u64) {
    // Add custom logic for winning the auction here
    transfer_to_identity(winning_amount, auction.asset_id, auction.beneficiary);
}
