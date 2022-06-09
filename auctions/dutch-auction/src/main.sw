contract;

dep abi;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{msg_sender, AuthError},
    context::{msg_amount, call_frames::msg_asset_id},
    result::*,
    contract_id::ContractId,
    identity::Identity,
    revert::revert,
    storage::StorageMap,
    token::transfer_to_output,
};

use abi::DutchAuction;

struct Auction {
    /// Price at the very start, usually higher than any expected price of sale
    opening_price: u64,
    /// The Price that the auction will eventually reach if no bids are recieved. Can also be used as the reserve price
    reserve_price: u64,
    /// From what point will the bids be allowed + from what point the price will start to drop
    start_time: u64,
    /// Only used for calculation of the price, users can still bid past this time for reserve_price unless its ended by the admin
    end_time: u64,
    /// The beneficiary of the proceeds of the auction
    beneficiary: Address,
    /// Whether or not the auction has ended already (Different from end_time, admin can prematurely end the auction.)
    ended: bool,
}


storage {
    /// Whether or not the constructor function has been called yet
    constructed: bool,
    /// Mapping an auction_id to its respective auction, allowing for multiple auctions to happen simultaneously
    auctions: StorageMap<u64, Auction> = StorageMap::new(),
    /// The Admin Address
    admin: Address,
    /// You can change this in the constructor, by default its ETH/AssetId 0
    asset_id: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    /// Tracking how many auctions have been made till now
    latest_auction_id: u64
}

enum Error {
    ContractNotConstructedYet: (),
    SenderNotAdmin: (),
    AuctionInProgress: (),
    AuctionAlreadyEnded: (),
    BidTooLow: (),
    WrongAssetSent: (),
    EndPriceCannotBeLargerThanStartPrice: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    AuctionCannotEndBeforeItStarts: (),
    AuctionNotYetStarted: (),
}

impl DutchAuction for Contract {
    fn constructor(admin: Address, asset: ContractId) {
        storage.constructed = true;
        storage.asset_id = asset;

        storage.beneficiary = admin;
        storage.admin = admin;
    }

    fn get_price(auction_id: u64) -> u64 {
        calculate_price(auction_id)
    }

    fn set_beneficiary(new_beneficiary: Address, auction_id: u64) {
        require(storage.constructed == true, Error::ContractNotConstructedYet);
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        storage.auctions.get(auction_id).beneficiary = new_beneficiary;
    }

    fn bid(auction_id: u64) {
        /// Since this is a dutch auction, first bid wins

        require(storage.constructed == true, Error::ContractNotConstructedYet);

        /// Checks for correct asset_id being sent and high enough amount being sent
        require(msg_asset_id() == storage.asset_id, Error::WrongAssetSent);
        require(msg_amount() >= calculate_price(auction_id), Error::BidTooLow);

        /// Cannot bid before auction starts
        require(height() >= storage.auctions.get(auction_id).start_time, Error::AuctionNotYetStarted);
        
        /// If ended == true, someone already bid or the admin prematurely ended the auction
        require(!storage.auctions.get(auction_id).ended, Error::AuctionAlreadyEnded);

        /// Disallows furthur bids
        storage.auctions.get(auction_id).ended = true;

        /// If someone sends more than the current price, refund the extra amount 
        if msg_amount() > calculate_price(auction_id) {
            let return_amount = msg_amount() - calculate_price(auction_id);
            transfer_to_output(return_amount, storage.asset_id, get_sender());
        }

        /// Logic on win put into the win function. Using a function here so that its easier to modify the logic 
        /// post-auction-win and so this contract template can be reused for multiple things
        win(auction_id);
    }

    fn setup_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64) -> u64 {
        require(storage.constructed == true, Error::ContractNotConstructedYet);

        require(get_sender() == storage.admin, Error::SenderNotAdmin);
        require(storage.ended == true, Error::AuctionInProgress);
        require(opening_price > reserve_price, Error::EndPriceCannotBeLargerThanStartPrice);
        require(end_time > height(), Error::AuctionCannotEndInThePast);
        require(start_time >= height(), Error::AuctionCannotStartInThePast);
        require(end_time > start_time, Error::AuctionCannotEndBeforeItStarts);

        let latest_auction = storage.latest_auction_id;

        storage.auctions.get(latest_auction).opening_price = opening_price;
        storage.auctions.get(latest_auction).reserve_price = reserve_price;
        storage.auctions.get(latest_auction).start_time = start_time;
        storage.auctions.get(latest_auction).end_time = end_time;

        storage.latest_auction_id = latest_auction + 1;

        /// Returns the auction being setup's auction_id
        storage.latest_auction_id
    }

    fn end_auction() {
        require(storage.constructed == true, Error::ContractNotConstructedYet);

        /// Only the admin can end the auction (prematurely)
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        /// If there is no auction going on currently the ended value will already be true so no need to check for that case
        storage.ended = true;
    }

}

fn win(auction_id: u64) {
    ///  Do stuff on a winning bid, this function is called whenever a winning bid is recieved.

    /// Add whatever logic you may want to execute on a win

    //Currently just sends the bid amount to the beneficiary
    transfer_to_output(calculate_price(auction_id), storage.asset_id, storage.auctions.get(auction_id).beneficiary);
}

fn calculate_price(auction_id: u64) -> u64 {
    /// How much the price will go down by, throughout the auction
    let price_delta = storage.auctions.get(auction_id).opening_price - storage.auctions.get(auction_id).reserve_price;
    /// How long the auction will last
    let auction_duration = storage.auctions.get(auction_id).end_time - storage.auctions.get(auction_id).start_time;
    /// This is the amount the price will reduce by per block
    let price_shift = price_delta / auction_duration;

    /// Tells us how far we are into the auction (out of the auction_duration)
    let now = height() - storage.auctions.get(auction_id).start_time; 

    /// Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let now = if now > auction_duration { auction_duration } else { now };

    /// price_shift * now tells us how much the price has reduced by now
    return storage.auctions.get(auction_id).opening_price - (price_shift * now);
}

/// Helper function to avoid having to repeat this code
fn get_sender() -> Address {
    let a: Result<Identity, AuthError> = msg_sender();
    let b = a.unwrap();

    let addy = match b {
        Identity::Address(addr) => addr,
        _ => revert(0),
    };
    addy
}
