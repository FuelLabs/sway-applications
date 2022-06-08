contract;

dep abi;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::msg_sender,
    context::{msg_amount, call_frames::msg_asset_id},
    result::*,
    contract_id::ContractId,
    identity::Identity,
    revert::revert,
    storage::StorageMap,
    token::transfer_to_output,
};

use abi::DutchAuction;


storage {
    /// Whether or not the constructor function has been called yet
    constructed: bool,
    /// Price at the very start, usually higher than any expected price of sale
    startingPrice: u64,
    /// The Price that the auction will eventually reach if no bids are recieved. Can also be used as the reserve price
    endingPrice: u64,
    /// From what point will the bids be allowed + from what point the price will start to drop
    startTime: u64,
    /// Only used for calculation of the price, users can still bid past this time for endingPrice unless its ended by the admin
    endTime: u64,
    /// The beneficiary of the proceeds of the auction
    beneficiary: Address,
    /// The Admin Address
    admin: Address,
    /// Whether or not the auction has ended already (Different from endTime, admin can prematurely end the auction.)
    ended: bool,
    /// You can change this in the constructor, by default its ETH/AssetId 0
    asset_id: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
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

fn win() {
    // Do stuff on the win event

    //Currently just sends the bid amount to the beneficiary
    transfer_to_output(price(), storage.asset_id, storage.beneficiary);
}

impl DutchAuction for Contract {
    fn get_price() -> u64 {
        return price();
    }

    fn set_beneficiary(new_beneficiary: Address) {
        require(storage.constructed == true, Error::ContractNotConstructedYet);
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        storage.beneficiary = new_beneficiary;
    }

    fn bid() {
        /// Since this is a dutch auction, first bid wins

        require(storage.constructed == true, Error::ContractNotConstructedYet);

        /// Checks for correct asset_id being sent and high enough amount being sent
        require(msg_asset_id() == storage.asset_id, Error::WrongAssetSent);
        require(msg_amount() >= price(), Error::BidTooLow);

        /// Cannot bid before auction starts
        require(height() >= storage.startTime, Error::AuctionNotYetStarted)
        
        /// If ended == true, someone already bid or the admin prematurely ended the auction
        require(!storage.ended, Error::AuctionAlreadyEnded);

        /// Disallows furthur bids
        storage.ended = true;

        /// If someone sends more than the current price, refund the extra amount 
        if msg_amount() > price() {
            let return_amount = msg_amount() - price();
            transfer_to_output(return_amount, storage.asset_id, get_sender());
        }

        /// Logic on win put into the win function
        win();
    }

    fn setup_auction(startp: u64, endp: u64, startt: u64, endt: u64) {
        require(storage.constructed == true, Error::ContractNotConstructedYet);

        require(get_sender() == storage.admin, Error::SenderNotAdmin);
        require(storage.ended == true, Error::AuctionInProgress);
        require(startp > endp, Error::EndPriceCannotBeLargerThanStartPrice);
        require(endt > height(), Error::AuctionCannotEndInThePast);
        require(startt > height(), Error::AuctionCannotStartInThePast);
        require(endt > startt, Error::AuctionCannotEndBeforeItStarts);

        storage.startingPrice = startp;
        storage.endingPrice = endp;
        storage.startTime = startt;
        storage.endTime = endt;
        storage.ended = false;
    }

    fn end_auction() {
        require(storage.constructed == true, Error::ContractNotConstructedYet);

        /// Only the admin can end the auction (prematurely)
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        /// If there is no auction going on currently the ended value will already be true so no need to check for that case
        storage.ended = true;
    }

    fn constructor(admin: Address, asset: ContractId) {
        storage.constructed = true;
        storage.asset_id = asset;

        storage.beneficiary = admin;
        storage.admin = admin;
    }
}

fn price() -> u64 {
    let price_difference = storage.startingPrice - storage.endingPrice;
    let duration = storage.endTime - storage.startTime;
    // This is the amount the price will reduce by per block
    let price_shift = price_difference / duration;

    let now = height() - storage.startTime; //Current block height - start will tell us how far we are into the auction now
    //Cap how far we are into the auction by the duration, so price doesnt go into negative or below endprice
    let now = if now > duration {
        duration
    } else {
        now
    };

    //price_shift * now tells us how much the price has reduced by now
    return storage.startingPrice - (price_shift * now);
}

fn get_sender() -> Address {
    // For some reason msg_sender().unwrap() doesnt work
    let unwrapped = 
    if let Result::Ok(inner_value) = msg_sender() {
            inner_value
    } else {
            revert(0);
    };

    let ad = if let Identity::Address(addr) = unwrapped {
        addr
    } else {
        revert(0);
    };
    ad
}
