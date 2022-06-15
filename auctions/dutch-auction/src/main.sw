contract;

dep abi;
dep data_structures;
dep errors;
dep events;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

use abi::DutchAuction;
use data_structures::Auction;
use errors::{SetupError, TimeError, UserError};
use events::{CancelledAuctionEvent, CreatedAuctionEvent, WinningBidEvent};

storage {
    /// Mapping an auction_id to its respective auction, allowing for multiple auctions to happen simultaneously
    auctions: StorageMap<u64,
    Auction>, /// Tracking how many auctions have been made till now
    auction_count: u64,
    /// Auction ids of the active auctions per beneficiary
    active_auctions_by_identity: StorageMap<Identity,
    [u64;
    1]>
}

impl DutchAuction for Contract {
    /// Returns the current price for the auction corresponding to the auction_id
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or higher than storage.auction_count
    fn price(auction_id: u64) -> u64 {
        validate_id(auction_id);
        calculate_price(auction_id)
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
    fn bid(auction_id: u64) {
        // In a Dutch auction the first bid wins
        validate_id(auction_id);

        let mut auction = storage.auctions.get(auction_id);

        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        // Cannot bid before auction starts
        require(auction.start_time <= height(), TimeError::AuctionNotYetStarted);
        // Cannot bid after auction ends
        require(height() <= auction.end_time, TimeError::AuctionAlreadyEnded);

        // Checks for correct asset_id being sent
        require(msg_asset_id() == auction.asset_id, UserError::WrongAssetSent);

        let price = calculate_price(auction_id);

        // Checks for high enough amount being sent
        require(price <= msg_amount(), UserError::BidTooLow);

        // Disallows furthur bids
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);

        // If someone sends more than the current price, refunds the extra amount
        if msg_amount() > price {
            let return_amount = msg_amount() - price;
            transfer_to_identity(return_amount, auction.asset_id, sender_indentity());
        }

        on_win(auction, price);

        /// WARNING: This needs to be changed to a pop to a vec instead of just replacing the contents of the array
        storage.active_auctions_by_identity.insert(auction.beneficiary, [0]);

        log(WinningBidEvent {
            id: auction_id, winner: sender_indentity(),  
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
    fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Identity, asset: ContractId) {
        require(reserve_price <= opening_price, SetupError::EndPriceCannotBeLargerThanStartPrice);
        require(height() < end_time, SetupError::AuctionCannotEndInThePast);
        require(height() <= start_time, SetupError::AuctionCannotStartInThePast);
        require(start_time < end_time, SetupError::AuctionCannotEndBeforeItStarts);

        let auction = Auction {
            opening_price, reserve_price, start_time, end_time, beneficiary, asset_id: asset,
            ended: false,
        };

        storage.auction_count = storage.auction_count + 1;
        storage.auctions.insert(storage.auction_count, auction);
        /// WARNING: This needs to be changed to a push to a vec instead of just replacing the contents of the array
        storage.active_auctions_by_identity.insert(beneficiary, [storage.auction_count]);

        log(CreatedAuctionEvent {
            auction, id: storage.auction_count, 
        });
    }

    /// Cancels an auction preventing any bids from being placed
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the beneficiary of the auction
    ///     3. auction has already ended
    fn cancel_auction(auction_id: u64) {
        validate_id(auction_id);

        let mut auction = storage.auctions.get(auction_id);

        // Only the beneficiary can end the auction (prematurely)
        require(eq_identity(sender_indentity(), auction.beneficiary), UserError::SenderNotBeneficiary);
        // Cannot cancel an auction that has already ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.ended = true;
        storage.auctions.insert(auction_id, auction);
        /// WARNING: This needs to be changed to a pop to a vec instead of just replacing the contents of the array
        storage.active_auctions_by_identity.insert(sender_indentity(), [0]);

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
    fn auction(auction_id: u64) -> Auction {
        validate_id(auction_id);
        storage.auctions.get(auction_id)
    }

    /// Changes the asset an auction accepts for a bid
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the beneficiary of the auction
    ///     3. auction has already ended
    fn change_asset(new_asset: ContractId, auction_id: u64) {
        validate_id(auction_id);
        let mut auction = storage.auctions.get(auction_id);

        // Only the beneficiary can change the bidding asset
        require(eq_identity(sender_indentity(), auction.beneficiary), UserError::SenderNotBeneficiary);
        // Cannot edit an auction that has ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.asset_id = new_asset;

        storage.auctions.insert(auction_id, auction);
    }

    /// Changes the beneficiary of the given auction
    ///
    /// # Panics
    ///
    /// This function will panic when:
    ///     1. auction_id is 0 or greater than storage.auction_count
    ///     2. msg_sender is not the beneficiary of the auction
    ///     3. auction has already ended
    fn change_beneficiary(new_beneficiary: Identity, auction_id: u64) {
        validate_id(auction_id);
        let mut auction = storage.auctions.get(auction_id);

        // Only the beneficiary can change the beneficiary
        require(eq_identity(sender_indentity(), auction.beneficiary), UserError::SenderNotBeneficiary);
        // Cannot edit an auction that has ended
        require(!auction.ended, TimeError::AuctionAlreadyEnded);

        auction.beneficiary = new_beneficiary;
        /// WARNING: This needs to be changed to a pop to a vec instead of just replacing the contents of the array
        storage.active_auctions_by_identity.insert(sender_indentity(), [0]);
        /// WARNING: This needs to be changed to a push to a vec instead of just replacing the contents of the array
        storage.active_auctions_by_identity.insert(new_beneficiary, [auction_id]);

        storage.auctions.insert(auction_id, auction);
    }

    /// Returns the active auctions of which the identity is the beneficiary of
    fn active_auctions_by_identity(identity_to_check: Identity) -> [u64;
    1] {
        storage.active_auctions_by_identity.get(identity_to_check)
    }
}

/// This function is called whenever a winning bid is recieved.
fn on_win(auction: Auction, winning_amount: u64) {
    // Add custom logic for winning the auction here
    transfer_to_identity(winning_amount, auction.asset_id, auction.beneficiary);
}

/// Calculates the current price of a given auction
fn calculate_price(auction_id: u64) -> u64 {
    let auction = storage.auctions.get(auction_id);

    // How much the price will go down by, throughout the auction
    let price_delta = auction.opening_price - auction.reserve_price;
    // How long the auction will last
    let auction_duration = auction.end_time - auction.start_time;
    // This is the amount the price will reduce by per block
    let price_shift = price_delta / auction_duration;

    // Tells us how far we are into the auction (out of the auction_duration)
    let blocks_into_auction = height() - auction.start_time;

    // Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let blocks_into_auction = if blocks_into_auction > auction_duration {
        auction_duration
    } else {
        blocks_into_auction
    };

    // price_shift * blocks_into_auction tells us how much the price has reduced by now
    auction.opening_price - (price_shift * blocks_into_auction)
}

/// Helper function to avoid having to repeat this code
fn sender_indentity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

/// Helper function to transfer assets to an identity
fn transfer_to_identity(amount: u64, asset_id: ContractId, reciever: Identity) {
    match reciever {
        Identity::Address(address) => {
            transfer_to_output(amount, asset_id, address);
        },
        Identity::ContractId(contractid) => {
            force_transfer_to_contract(amount, asset_id, contractid);
        },
    };
}

/// Helper function to compare identities
fn eq_identity(id_1: Identity, id_2: Identity) -> bool {
    match id_1 {
        Identity::Address(address1) => {
            match id_2 {
                Identity::Address(address2) => {
                    address1 == address2
                },
                _ => false, 
            }
        },
        Identity::ContractId(contract_id_1) => {
            match id_2 {
                Identity::ContractId(contract_id_2) => {
                    contract_id_1 == contract_id_2
                },
                _ => false, 
            }
        },
    }
}

/// Validates an auction_id to make sure it corresponds to an auction
fn validate_id(id: u64) {
    // If the id is greater than the auction count then it's invalid
    require(id != 0, UserError::InvalidAuctionID);
    require(id <= storage.auction_count, UserError::InvalidAuctionID);
}
