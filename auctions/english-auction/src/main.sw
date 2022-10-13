contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::{Asset, Auction, State};
use errors::{AccessError, InitError, InputError, UserError};
use events::{BidEvent, CancelAuctionEvent, CreateAuctionEvent, WithdrawEvent};
use interface::{EnglishAuction, NFT};
use utils::{approved_for_nft_transfer, owns_nft, transfer_asset, transfer_nft, validate_asset};

use std::{
    block::height,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    context::{
        call_frames::{
            contract_id,
            msg_asset_id,
        },
        msg_amount,
    },
    logging::log,
    storage::StorageMap,
};

storage {
    /// Stores the auction information based on auction ID
    /// Map(auction_id => auction)
    auctions: StorageMap<u64, Option<Auction>> = StorageMap {}, 
    
    // TODO: Move deposits into the Auction struct when StorageMaps are
    //       supported inside structs
    ///
    deposits: StorageMap<(Identity, u64), Option<Asset>> = StorageMap {},
    /// The total number of auctions that have been created
    /// This should only be incremented
    total_auctions: u64 = 0,
}

impl EnglishAuction for Contract {
    #[storage(read)]
    fn auction_info(auction_id: u64) -> Option<Auction> {
        storage.auctions.get(auction_id)
    }

    #[storage(read, write)]
    fn bid(auction_id: u64, bid_asset: Asset) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Make sure this auction is open to taking bids
        require(auction.state == State::Open && height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        // Ensure the `bid_asset` struct has the correct contract_id, the transaction's amount
        // is correct, and if it's an NFT we can transfer it to this auction contract
        validate_asset(auction.bid_asset, bid_asset);

        // The bidder cannot be the seller
        let sender = msg_sender().unwrap();
        require(sender != auction.seller, UserError::BidderIsSeller);

        // Combine the user's previous deposits and the current bid for the
        // total bid the user has made
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let total_bid_asset = match sender_deposit {
            Option::Some(Asset) => {
                bid_asset + sender_deposit.unwrap()
            },
            Option::None(Asset) => {
                bid_asset
            }
        };

        // Make sure this is greater than initial bid and this bid plus the previously placed bids are more than the current bid
        require(total_bid_asset.amount() >= auction.initial_price, InputError::InitialPriceNotMet);
        require(total_bid_asset.amount() > auction.bid_asset.amount(), InputError::IncorrectAmountProvided);

        // Check to see if we've reached the reserve price if there is one
        let reserve: Option<u64> = auction.reserve_price;
        if (reserve.is_some()) {
            // The bid cannot be greater than the reserve price
            require(reserve.unwrap() >= total_bid_asset.amount(), InputError::IncorrectAmountProvided);
            if (reserve.unwrap() == total_bid_asset.amount()) {
                // The reserve price was met
                auction.state = State::Closed;
            }
        }

        // Finally, make the bid
        // Transfer any NFTs to this contract
        match total_bid_asset {
            Asset::NFTAsset(nft_asset) => {
                // We need to transfer ownership to the auction contract if they are
                // bidding an NFT
                transfer_nft(sender, Identity::ContractId(contract_id()), nft_asset);
            }
            _ => {}
        }

        // Update the auction's information
        auction.highest_bidder = Option::Some(sender);
        auction.bid_asset = total_bid_asset;

        // Store the new auction information and the user's deposit
        storage.deposits.insert((sender, auction_id), Option::Some(auction.bid_asset));
        storage.auctions.insert(auction_id, Option::Some(auction));

        // Log the bid
        log(BidEvent {
            amount: auction.bid_asset.amount(),
            auction_id: auction_id,
            identity: sender,
        });
    }

    #[storage(read, write)]
    fn cancel(auction_id: u64) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // TODO: Should not be able to cancel auction if the auction is closed

        // The sender has to be the seller in order to cancel their auction
        require(msg_sender().unwrap() == auction.seller, AccessError::SenderIsNotSeller);

        // Update and store the auction's information
        auction.highest_bidder = Option::None();
        auction.state = State::Closed;
        storage.auctions.insert(auction_id, Option::Some(auction));

        // Log that the auction was canceled
        log(CancelAuctionEvent { auction_id });
    }

    #[storage(read, write)]
    fn create(
        bid_asset: Asset,
        duration: u64,
        initial_price: u64,
        reserve_price: Option<u64>,
        seller: Identity,
        sell_asset: Asset,
    ) -> u64 {
        // Either there is no reserve price or the reserve must be greater than the initial price
        require(reserve_price.is_none() || (reserve_price.is_some() && reserve_price.unwrap() >= initial_price && reserve_price.unwrap() != 0), InitError::ReserveLessThanInitialPrice);
        // The auction must last for some time
        require(duration != 0, InitError::AuctionTimeNotProvided);

        // Ensure that the `sell_asset` struct and what was sent in the transaction match
        match sell_asset {
            Asset::TokenAsset(asset) => {
                // Selling tokens
                require(msg_amount() == asset.amount, InputError::IncorrectAmountProvided);
                require(msg_asset_id() == asset.contract_id, InputError::IncorrectAssetProvided);
            },
            Asset::NFTAsset(asset) => {
                // Selling NFTs
                // Ensure that the sender is the owner
                let sender = msg_sender().unwrap();
                require(owns_nft(sender, asset), AccessError::NFTTransferNotApproved);

                // Ensure that the auction contract can transfer the NFT tokens to itself
                require(approved_for_nft_transfer(sender, Identity::ContractId(contract_id()), asset), AccessError::NFTTransferNotApproved);

                // Transfer NFT tokens to this contract
                transfer_nft(seller, Identity::ContractId(contract_id()), asset);
            }
        }

        // Setup auction
        let auction = Auction {
            bid_asset: bid_asset,
            highest_bidder: Option::None(),
            end_block: height() + duration,
            initial_price: initial_price,
            reserve_price: reserve_price,
            sell_asset: sell_asset,
            seller: seller,
            state: State::Open,
        };

        // Store the auction information
        storage.deposits.insert((seller, storage.total_auctions), Option::Some(sell_asset));
        storage.auctions.insert(storage.total_auctions, Option::Some(auction));

        // Log the start of the new auction
        log(CreateAuctionEvent {
            auction_id: storage.total_auctions,
        });

        // Return the auction ID and increment the total auctions counter
        storage.total_auctions += 1;
        storage.total_auctions - 1
    }

    #[storage(read)]
    fn deposit(auction_id: u64, identity: Identity) -> Option<Asset> {
        storage.deposits.get((identity, auction_id))
    }

    #[storage(read, write)]
    fn withdraw(auction_id: u64) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Cannot withdraw if the auction is still on going
        require(auction.state == State::Closed || height() >= auction.end_block, AccessError::AuctionIsNotClosed);

        // If time has run out set the contract state to closed
        if (height() >= auction.end_block
            && auction.state == State::Open)
        {
            auction.state = State::Closed;
            storage.auctions.insert(auction_id, Option::Some(auction));
        }

        // Set some variables we will need
        let sender = msg_sender().unwrap();
        let bidder: Option<Identity> = auction.highest_bidder;
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));

        // Make sure the sender still has something to withdraw
        require(sender_deposit.is_some(), UserError::UserHasAlreadyWithdrawn);
        storage.deposits.insert((sender, auction_id), Option::None());
        let mut withdrawn_asset = sender_deposit.unwrap();

        // Go ahead and withdraw
        if ((bidder.is_some()
            && sender == bidder.unwrap())
            || sender == auction.seller)
        {
            // The buyer is withdrawing or the seller is withdrawing and no one placed a bid
            transfer_asset(sender, auction.sell_asset);
            withdrawn_asset = auction.sell_asset;
        } else if (sender == auction.seller) {
            // The seller is withdrawing and there was a winning bidder
            transfer_asset(sender, auction.bid_asset);
            withdrawn_asset = auction.bid_asset;
        } else {
            // Anyone with a failed bid is withdrawing
            transfer_asset(sender, sender_deposit.unwrap());
        };

        // Log the withdrawal
        log(WithdrawEvent {
            amount: withdrawn_asset.amount(),
            auction_id,
            identity: sender,
        });
    }

    #[storage(read)]
    fn total_auctions() -> u64 {
        storage.total_auctions
    }
}
