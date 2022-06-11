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
use errors::Error;
use events::{AuctionCancelledEvent, CreatedAuctionEvent, WinningBidEvent};

storage {
    /// The Admin Address
    admin: Identity,
    /// Mapping an auction_id to its respective auction, allowing for multiple auctions to happen simultaneously
    auctions: StorageMap<u64,
    Auction>, /// Tracking how many auctions have been made till now
    auction_count: u64,
    /// Whether the constructor has been called
    initialized: bool,
}

impl DutchAuction for Contract {
    fn constructor(admin: Identity) {
        require(!storage.initialized, Error::CannotReinitialize);
        storage.admin = admin;
        storage.initialized = true;
    }

    fn price(auction_id: u64) -> u64 {
        validate_id(auction_id);

        calculate_price(auction_id)
    }

    fn bid(auction_id: u64) {
        // In a Dutch auction the first bid wins
        require(storage.initialized, Error::ContractNotYetInitialized);

        validate_id(auction_id);

        let mut auction = storage.auctions.get(auction_id);

        require(!auction.ended, Error::AuctionAlreadyEnded);

        // Cannot bid before auction starts
        require(auction.start_time <= height(), Error::AuctionNotYetStarted);

        // Checks for correct asset_id being sent
        require(msg_asset_id() == auction.asset_id, Error::WrongAssetSent);

        let price = calculate_price(auction_id);

        // Checks for high enough amount being sent
        require(price <= msg_amount(), Error::BidTooLow);

        // Disallows furthur bids
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);

        // If someone sends more than the current price, refunds the extra amount
        if msg_amount() > price {
            let return_amount = msg_amount() - price;
            transfer_to_identity(return_amount, auction.asset_id, get_sender_identity());
        }

        on_win(auction_id, price);

        log(WinningBidEvent {
            winner: get_sender_identity(), id: auction_id, 
        });
    }

    fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Identity, asset: ContractId) {
        require(storage.initialized, Error::ContractNotYetInitialized);
        require(eq_identity(storage.admin, get_sender_identity()), Error::SenderNotAdmin);
        require(reserve_price <= opening_price, Error::EndPriceCannotBeLargerThanStartPrice);
        require(height() < end_time, Error::AuctionCannotEndInThePast);
        require(height() <= start_time, Error::AuctionCannotStartInThePast);
        require(start_time < end_time, Error::AuctionCannotEndBeforeItStarts);

        storage.auction_count = storage.auction_count + 1;

        let auction = Auction {
            opening_price, reserve_price, start_time, end_time, beneficiary, asset_id: asset,
            ended: false,
        };

        storage.auctions.insert(storage.auction_count, auction);

        log(CreatedAuctionEvent {
            id: storage.auction_count, auction, 
        });
    }

    fn cancel_auction(auction_id: u64) {
        require(storage.initialized, Error::ContractNotYetInitialized);

        validate_id(auction_id);

        // Only the admin can end the auction (prematurely)
        require(eq_identity(get_sender_identity(), storage.admin), Error::SenderNotAdmin);

        let mut auction = storage.auctions.get(auction_id);
        auction.ended = true;
        storage.auctions.insert(auction_id, auction);

        log(AuctionCancelledEvent {
            id: auction_id, 
        });
    }
}

/// This function is called whenever a winning bid is recieved.
fn on_win(auction_id: u64, winning_amount: u64) {
    let auction = storage.auctions.get(auction_id);
    transfer_to_identity(winning_amount, auction.asset_id, auction.beneficiary);
}

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
fn get_sender_identity() -> Identity {
    let a: Result<Identity, AuthError> = msg_sender();
    let b = a.unwrap();

    b
}

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

fn validate_id(id: u64) {
    // If the given auction id is higher than the auction count, its an invalid auction_id
    require(id != 0, Error::InvalidAuctionID);
    require(id <= storage.auction_count, Error::InvalidAuctionID);
}
