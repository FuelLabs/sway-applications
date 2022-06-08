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

//Set this to your own address
const MY_ADDRESS: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

storage {
    startingPrice: u64,
    endingPrice: u64,
    startTime: u64,
    /// Only used for calculation of the price, users can still bid past this time for endingPrice unless its ended by the admin
    endTime: u64,
    /// The beneficiary of the proceeds of the auction
    beneficiary: Address = ~Address::from(MY_ADDRESS),
    /// The Admin Address
    admin: Address = ~Address::from(MY_ADDRESS),
    /// Whether or not the auction has ended already (Different from endTime, admin can prematurely end the auction.)
    ended: bool,
    /// You can change this in the setup, by default its ETH/AssetId 0
    asset_id: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

enum Error {
    SenderNotAdmin: (),
    AuctionInProgress: (),
    AuctionAlreadyEnded: (),
    BidTooLow: (),
    WrongAssetSent: (),
    EndPriceCannotBeLargerThanStartPrice: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    AuctionCannotEndBeforeItStarts: (),
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
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        storage.beneficiary = new_beneficiary;
    }

    fn bid() {
        //Since this is the dutch auction, first bid wins

        require(msg_asset_id() == storage.asset_id, Error::WrongAssetSent);
        require(msg_amount() >= price(), Error::BidTooLow);
        
        require(!storage.ended, Error::AuctionAlreadyEnded);

        //Disallows furthur bids
        storage.ended = true;

        if msg_amount() > price() {
            let return_amount = msg_amount() - price();
            transfer_to_output(return_amount, storage.asset_id, get_sender());
        }

        win();
    }

    fn setup_auction(startp: u64, endp: u64, startt: u64, endt: u64, asset: ContractId) {
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
        storage.asset_id = asset;
    }

    fn end_auction() {
        /// Only the admin can end the auction (prematurely)
        require(get_sender() == storage.admin, Error::SenderNotAdmin);

        /// If there is no auction going on currently the ended value will already be true so no need to check for that case
        storage.ended = true;
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
