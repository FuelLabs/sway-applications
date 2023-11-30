contract;

mod errors;
mod data_structures;
mod events;
mod interface;

use ::data_structures::{auction::Auction, state::State};
use ::errors::{AccessError, InitError, InputError, UserError};
use ::events::{BidEvent, CancelAuctionEvent, CreateAuctionEvent, WithdrawEvent};
use ::interface::{EnglishAuction, Info};
use std::{
    auth::msg_sender,
    block::height,
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    context::msg_amount,
    hash::Hash,
    token::transfer,
};

storage {
    /// Stores the auction information based on auction ID.
    /// Map(auction id => auction)
    auctions: StorageMap<u64, Auction> = StorageMap {},
    // TODO: Move deposits into the Auction struct when StorageMaps are supported inside structs
    // This issue can be tracked here: https://github.com/FuelLabs/sway/issues/2465
    /// Stores the deposits made based on the user and auction ID.
    /// Map((user, auction id) => deposit)
    deposits: StorageMap<(Identity, u64), u64> = StorageMap {},
    /// The total number of auctions that have ever been created.
    total_auctions: u64 = 0,
}

impl EnglishAuction for Contract {
    #[payable]
    #[storage(read, write)]
    fn bid(auction_id: u64) {
        let auction = storage.auctions.get(auction_id).try_read();
        require(auction.is_some(), InputError::AuctionDoesNotExist);

        let mut auction = auction.unwrap();
        let sender = msg_sender().unwrap();
        let bid_asset = msg_asset_id();
        let bid_amount = msg_amount();
        require(sender != auction.seller, UserError::BidderIsSeller);
        require(
            auction
                .state == State::Open && auction
                .end_block >= height(),
            AccessError::AuctionIsNotOpen,
        );
        require(
            bid_asset == auction
                .bid_asset,
            InputError::IncorrectAssetProvided,
        );

        // Combine the user's previous deposits and the current bid for the
        // total deposits to the auction the user has made
        let total_bid = match storage.deposits.get((sender, auction_id)).try_read() {
            Some(sender_deposit) => {
                bid_amount + sender_deposit
            },
            None => {
                bid_amount
            }
        };

        require(
            total_bid >= auction
                .initial_price,
            InputError::InitialPriceNotMet,
        );
        require(
            total_bid > auction
                .highest_bid,
            InputError::IncorrectAmountProvided,
        );

        // Check if reserve has been met if there is one set
        if auction.reserve_price.is_some() {
            // The bid cannot be greater than the reserve price
            let reserve_price = auction.reserve_price.unwrap();
            require(
                reserve_price >= total_bid,
                InputError::IncorrectAmountProvided,
            );

            if reserve_price == total_bid {
                auction.state = State::Closed;
            }
        }

        // Update the auction's information and store the new state
        auction.highest_bidder = Option::Some(sender);
        auction.highest_bid = total_bid;
        storage.deposits.insert((sender, auction_id), total_bid);
        storage.auctions.insert(auction_id, auction);

        log(BidEvent {
            amount: auction.highest_bid,
            auction_id: auction_id,
            user: sender,
        });
    }

    #[storage(read, write)]
    fn cancel(auction_id: u64) {
        // Make sure this auction exists
        let auction = storage.auctions.get(auction_id).try_read();
        require(auction.is_some(), InputError::AuctionDoesNotExist);

        let mut auction = auction.unwrap();
        require(
            auction
                .state == State::Open && auction
                .end_block >= height(),
            AccessError::AuctionIsNotOpen,
        );
        require(
            msg_sender()
                .unwrap() == auction
                .seller,
            AccessError::SenderIsNotSeller,
        );

        // Update and store the auction's information
        auction.highest_bidder = Option::None;
        auction.state = State::Closed;
        storage.auctions.insert(auction_id, auction);

        log(CancelAuctionEvent { auction_id });
    }

    #[payable]
    #[storage(read, write)]
    fn create(
        bid_asset: AssetId,
        duration: u32,
        initial_price: u64,
        reserve_price: Option<u64>,
        seller: Identity,
    ) -> u64 {
        // Either there is no reserve price or the reserve must be greater than the initial price
        require(
            reserve_price
                .is_none() || (reserve_price
                .is_some() && reserve_price
                .unwrap() >= initial_price),
            InitError::ReserveLessThanInitialPrice,
        );
        require(duration != 0, InitError::AuctionDurationNotProvided);
        require(initial_price != 0, InitError::InitialPriceCannotBeZero);

        let sell_asset = msg_asset_id();
        let sell_asset_amount = msg_amount();
        require(msg_amount() != 0, InputError::IncorrectAmountProvided);

        // Setup auction
        let auction = Auction::new(
            bid_asset,
            duration + height(),
            initial_price,
            reserve_price,
            sell_asset,
            sell_asset_amount,
            seller,
        );

        // Store the auction information
        let total_auctions = storage.total_auctions.read();
        storage
            .deposits
            .insert((seller, total_auctions), sell_asset_amount);
        storage.auctions.insert(total_auctions, auction);

        log(CreateAuctionEvent {
            auction_id: total_auctions,
            bid_asset,
            sell_asset,
            sell_asset_amount,
        });

        storage
            .total_auctions
            .write(storage.total_auctions.read() + 1);
        total_auctions
    }

    #[storage(read, write)]
    fn withdraw(auction_id: u64) {
        // Make sure this auction exists
        let auction = storage.auctions.get(auction_id).try_read();
        require(auction.is_some(), InputError::AuctionDoesNotExist);

        // Cannot withdraw if the auction is still on going
        let mut auction = auction.unwrap();
        require(
            auction
                .state == State::Closed || auction
                .end_block <= height(),
            AccessError::AuctionIsNotClosed,
        );
        if (auction.end_block <= height()
            && auction.state == State::Open)
        {
            auction.state = State::Closed;
            storage.auctions.insert(auction_id, auction);
        }

        let sender = msg_sender().unwrap();
        let bidder = auction.highest_bidder;
        let sender_deposit = storage.deposits.get((sender, auction_id)).try_read();

        // Make sure the sender still has something to withdraw
        require(sender_deposit.is_some(), UserError::UserHasAlreadyWithdrawn);
        assert(storage.deposits.remove((sender, auction_id)));
        let mut withdrawn_amount = sender_deposit.unwrap();
        let mut withdrawn_asset = auction.bid_asset;

        // Withdraw owed assets
        if ((bidder.is_some() && sender == bidder.unwrap()) || (bidder.is_none() && sender == auction.seller)) {
            // Winning bidder or seller withdraws original sold assets
            transfer(sender, auction.sell_asset, auction.sell_asset_amount);
            withdrawn_asset = auction.sell_asset;
            withdrawn_amount = auction.sell_asset_amount;
        } else if (sender == auction.seller) {
            // Seller withdraws winning bids
            transfer(sender, auction.bid_asset, auction.highest_bid);
            withdrawn_amount = auction.highest_bid;
        } else {
            // Bidders withdraw failed bids
            transfer(sender, withdrawn_asset, withdrawn_amount);
        };

        log(WithdrawEvent {
            asset: withdrawn_asset,
            asset_amount: withdrawn_amount,
            auction_id,
            user: sender,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn auction_info(auction_id: u64) -> Option<Auction> {
        storage.auctions.get(auction_id).try_read()
    }

    #[storage(read)]
    fn deposit_balance(auction_id: u64, identity: Identity) -> Option<u64> {
        storage.deposits.get((identity, auction_id)).try_read()
    }

    #[storage(read)]
    fn total_auctions() -> u64 {
        storage.total_auctions.read()
    }
}
