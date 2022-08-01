contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::{Asset, Auction, State};
use errors::{AccessError, InitError, InputError, UserError};
use events::{CancelAuctionEvent, CreateAuctionEvent, BidEvent, WithdrawEvent};
use interface::{EnglishAuction, NFT};
use utils::{
    approved_for_nft_transfer,
    owns_nft,
    transfer_asset,
    transfer_nft,
    validate_asset,
};

use std::{
    revert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    storage::StorageMap,
};

storage {
    /// Stores the auction information based on auction ID
    /// Map(auction_id => auction)
    auctions: StorageMap<u64,
    Option<Auction>> = StorageMap { }, // TODO: Move deposits into the Auction struct when StorageMaps are
    //       supported inside structs
    ///
    deposits: StorageMap<(Identity,
    u64), Option<Asset>> = StorageMap { }, /// The total number of auctions that have been created
    /// This should only be incremented
    total_auctions: u64 = 0,
}

impl EnglishAuction for Contract {
    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
    /// Returns the auction struct for the corresponding auction id.
    /// If the auction does not exist `None` will be returned.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    // #[storage(read)]
    // fn auction_info(auction_id: u64) -> Option<Auction> {
    //     storage.auctions.get(auction_id)
    // }

    /// Places a bid on the auction specified. A correctly structured `Asset` struct must be
    /// provided. A bid is only valid if it is greater than the last bid or greater than the
    /// initial_price. If the reserve price is met, the auction will end.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    /// * `new_bid_asset` - An `Asset` enum that is either a `TokenAsset` struct or a `NFTAsset` struct.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the auction has closed.
    /// * When the bidding time for the auction has closed.
    /// * When the `new_bid_asset` `ContractId` provided does not match the auction's `bid_asset`
    ///   `ContractId`.
    /// * When the `new_bid_asset` amount provided does not match the transaction's `msg_amount`.
    /// * When the auction contract does not have permission to transfer the NFT to it's ownership.
    /// * When the bidder/sender is the auction's `seller`.
    /// * When the total of previous plus this bid is greater than the reserve price.
    /// * When the `new_bid_asset` amount provided is less than the initial price if there are no bids.
    /// * When the total of previous plus this bid amounts are not greater than the current bid
    ///   amount.
    #[storage(read, write)]fn bid(auction_id: u64, new_bid_asset: Asset) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Make sure this auction is open to taking bids
        require(auction.state == State::Open, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        // Ensure the `new_bid_asset` struct has the correct contract_id, the transaction's amount
        // is correct, and if it's an NFT we can transfer it to this auction contract
        validate_asset(auction.bid_asset, new_bid_asset);

        // The bidder cannot be the seller
        let sender =  msg_sender().unwrap();
        require(sender != auction.seller, UserError::BidderIsSeller);

        // Combine the user's previous deposits and the current bid for the
        // total bid the user has made
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let total_bid_asset = match sender_deposit {
            Option::Some(Asset) => {
                new_bid_asset + sender_deposit.unwrap()
            },
            Option::None(Asset) => {
                new_bid_asset
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
            _ => {
            }
        }

        // Update the auction's information
        auction.highest_bidder = Option::Some(sender);
        auction.bid_asset = total_bid_asset;

        // Store the new auction information and the user's deposit
        storage.deposits.insert((sender, auction_id), Option::Some(auction.bid_asset));
        storage.auctions.insert(auction_id, Option::Some(auction));

        // Log the bid
        log(BidEvent {
            amount: auction.bid_asset.amount(), auction_id: auction_id, identity: sender
        });
    }

    /// Cancels the specified auction. Once the auction has been canceled user will be able to
    /// withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` does not map to an existing auction.
    /// * When the `sender` is not the `seller` of the auction.
    #[storage(read, write)]fn cancel(auction_id: u64) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // The sender has to be the seller in order to cancel their auction
        require(msg_sender().unwrap() == auction.seller, AccessError::SenderIsNotSeller);

        // Update and store the auction's information
        auction.highest_bidder = Option::None();
        auction.state = State::Closed;
        storage.auctions.insert(auction_id, Option::Some(auction));

        // Log that the auction was canceled
        log(CancelAuctionEvent {
            auction_id
        });
    }

    /// Starts an auction with a seller, selling asset, buying asset, prices, and length of the
    /// auction. This function will return a `u64` for the created auction's ID number.
    ///
    /// # Arguments
    ///
    /// `seller` - The `Identity` of the seller for this auction. This `Identity` will have the
    ///            ability to cancel and withdraw the originially provided assets.
    /// `sell_asset` - The `Asset` struct that contains information about what is being auctioned
    ///                off.
    /// `bid_asset` - The `Asset` struct that contains the `contract_id` of the asset the seller is
    ///               willing to accept in return for the `sell_asset`.
    /// `initial_price` - The starting price at which the auction should start.
    /// `reserve_price` - The price at which a buyer may purchase the `sell_asset` outright.
    /// `time` - The duration of the auction in number of blocks.
    ///
    /// # Reverts
    ///
    /// * When the `initial_price` is higher than the `reserve_price` if a `reserve_price` is set.
    /// * When the `time` or duration of the auction is set to zero.
    /// * When the transaction's token amount tranfered is not the amount specified in the
    ///   `sell_asset` struct.
    /// * When the transaction's token `contract_id` is not the same as the `contract_id` specified
    ///   in the `sell_asset` struct.
    /// * When the `sender` is not the owner of the NFT's provided in the `sell_asset` struct.
    /// * When the auction contract is not approved to transfer the NFT's provided in the
    ///   `sell_asset` struct.
    #[storage(read, write)]fn create(seller: Identity, sell_asset: Asset, bid_asset: Asset, initial_price: u64, reserve_price: u64, time: u64) -> u64 {
        // Either there is no reserve price or the reserve must be greater than the initial price
        require((reserve_price >= initial_price && reserve_price != 0) || reserve_price == 0, InitError::ReserveLessThanInitialPrice);
        // The auction must last for some time
        require(time != 0, InitError::AuctionTimeNotProvided);

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

        // Does the seller want a reserve
        let reserve = match reserve_price {
            0 => Option::None(), _ => Option::Some(reserve_price), 
        };

        // Setup auction
        let auction = Auction {
            bid_asset: bid_asset,
            highest_bidder: Option::None(),
            end_block: height() + time,
            initial_price: initial_price,
            reserve_price: reserve,
            sell_asset: sell_asset,
            seller: seller,
            state: State::Open,
        };

        // Store the auction information
        storage.deposits.insert((seller, storage.total_auctions), Option::Some(sell_asset));
        storage.auctions.insert(storage.total_auctions, Option::Some(auction));

        // Log the start of the new auction
        log(CreateAuctionEvent {
            auction_id: storage.total_auctions
        });

        // Return the auction ID and increment the total auctions counter
        storage.total_auctions = storage.total_auctions + 1;
        storage.total_auctions - 1
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
    /// Returns the balance of the user's `bid_asset` deposits. If the user has not deposited any
    /// assets for the provided `auction_id` then `None` will be returned.
    ///
    /// # Arguments
    ///
    /// * `identity` - The `Identity` of the user which has deposited assets
    /// * `auction_id` - The `u64` id number of the auction.
    // #[storage(read)]
    // fn deposit(identity: Identity, auction_id: u64) -> Option<Asset> {
    //     storage.deposits.get((identity, auction_id))
    // }

    /// Allows users to withdraw their assets if the auction has gone over time, the reserve has
    /// been met, or been canceled. If there is a winning bidder, the winning bidder will withdraw
    /// the `sell_asset`, the failed bidders will withdraw their original deposit, and the seller
    /// will withdraw the winning bidder's deposit. If there is no winning bidder or the auction
    /// was canceled, the seller and bidder will withdraw their original deposits.
    ///
    /// # Arguments
    ///
    /// * `auction_id` - The `u64` id number of the auction.
    ///
    /// # Reverts
    ///
    /// * When the `auction_id` provided does not map to an existing auction.
    /// * When the duration of the auction has not ended.
    /// * When the auction's `state` is still in the open bidding state.
    /// * When the `sender` has already withdrawn their deposit.
    #[storage(read, write)]fn withdraw(auction_id: u64) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Cannot withdraw if the auction is still on going
        require(auction.state == State::Closed || height() >= auction.end_block, AccessError::AuctionIsNotClosed);

        // If time has run out set the contract state to closed
        if (height() >= auction.end_block && auction.state == State::Open) {
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
        if ((bidder.is_some() && sender == bidder.unwrap()) || sender == auction.seller) {
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
            amount: withdrawn_asset.amount(), auction_id, identity: sender
        });
    }

    /// Returns the total auctions which have been started using this auction contract.
    #[storage(read)]fn total_auctions() -> u64 {
        storage.total_auctions
    }
}
