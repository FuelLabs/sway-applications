contract;

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use abi::{EnglishAuction, NFT};
use data_structures::{Asset, Auction};
use errors::{AccessError, InitError, InputError, UserError};
use events::{AuctionStartEvent, BidEvent, WithdrawEvent};
use utils::{
    approved_for_nft_transfer, 
    identities_equal, reserve_met, 
    sender_identity, 
    send_tokens, 
    transfer_nft
};

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::*,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

storage {
    auctions: StorageMap<u64, Option<Auction>>,
    // TODO: Move deposits into the Auction struct when StorageMaps are 
    //       supported inside structs
    deposits: StorageMap<(Identity, u64), Option<Asset>>,
    total_auctions: u64,
}

impl EnglishAuction for Contract {

    /// Returns the block at which the auction will end
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]
    fn auction_end_block(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);

        let auction = auction.unwrap();
        auction.end_block
    }

    /// Places a bid on the auction specified. If the reserve is met the
    /// asset will be sold and the auction will be over. Any amount over
    /// the reserve is returned to the sender
    ///
    /// # Panics
    ///
    /// This function will panic when:
    /// - The auction does not exist
    /// - The auction is not in the bidding state
    /// - The auction is not open
    /// - The asset provided is not the buy asset
    /// - The asset amount provided is less than the inital price if there are no bids
    /// - The bidder is the seller
    /// - The asset amount provided plus current deposit is less than or equal to the current bid
    #[storage(read, write)]
    fn bid(auction_id: u64) -> bool {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        require(auction.state == 1, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);
        require(msg_asset_id() == auction.buy_asset.contract_id, InputError::IncorrectAssetProvided);

        let current_bid = auction.buy_asset.amount;
        if (current_bid == 0) {
            require(msg_amount() >= auction.inital_price, InputError::InitalPriceNotMet);
        }

        let sender: Identity = sender_identity();
        let nft_id: Option<u64> = auction.buy_asset.nft_id;
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        // TODO: Change this
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(BASE_ASSET_ID), nft_id: Option::None()},
        };
        
        require(!identities_equal(sender, auction.seller), UserError::BidderIsSeller);

        // TODO: Allow for bidding of mutliple NFTs
        if (nft_id.is_none()) {
            require(msg_amount() + sender_deposit.amount >= current_bid, InputError::IncorrectAmountProvided);
        }

        let reserve: Option<u64> = auction.reserve_price;
        if (reserve.is_none() || msg_amount() + sender_deposit.amount < reserve.unwrap()) {
            // If the reserve price has not yet been met
            auction.bidder = Option::Some(sender);
            auction.buy_asset.amount = sender_deposit.amount + msg_amount();
            storage.auctions.insert(auction_id, Option::Some(auction));
            storage.deposits.insert((sender, auction_id), Option::Some(auction.buy_asset));
            log(BidEvent{asset: auction.buy_asset, auction_id: auction_id, identity: sender});
        } else {
            // The reserve price was met
            let auction = reserve_met(auction, sender_deposit.amount, reserve.unwrap());
            storage.auctions.insert(auction_id, Option::Some(auction));
            storage.deposits.insert((sender, auction_id), Option::None());

            let bought_asset = Asset{
                amount: reserve.unwrap(),
                contract_id: auction.buy_asset.contract_id,
                nft_id: auction.buy_asset.nft_id
            };
            log(WithdrawEvent{asset: bought_asset, auction_id: auction_id, identity: sender});  
        }
        true
    }

    /// Purchases at the reserve price. If a deposit greater than the
    /// reserve is made, the rest will be returned
    ///
    /// # Panics
    /// 
    /// This function will panic when:
    /// - The auction does not exists
    /// - The auction is not in the bidding state
    /// - The auction is not open
    /// - There is no reserve price set
    /// - The bidder is the seller
    /// - The asset amount does not meet the reserve price
    /// - The buy assest provided is the incorrect asset
    #[storage(read, write)]
    fn buy_reserve(auction_id: u64) -> bool {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();

        require(auction.state == 1, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        let reserve: Option<u64> = auction.reserve_price;
        require(reserve.is_some(), AccessError::NoReserveSet);


        let sender = sender_identity();
        let nft_id: Option<u64> = auction.buy_asset.nft_id;
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        // TODO: Change this
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(BASE_ASSET_ID), nft_id: Option::None()},
        };

        require(!identities_equal(sender, auction.seller), UserError::BidderIsSeller);
        require(msg_asset_id() == auction.buy_asset.contract_id, InputError::IncorrectAssetProvided);

        // TODO: Allow for mutliple NFTs
        if (nft_id.is_none()) {
            require(msg_amount() + sender_deposit.amount >= reserve.unwrap(), InputError::IncorrectAmountProvided);
        }

        // The reserve price was met
        let auction = reserve_met(auction, sender_deposit.amount, reserve.unwrap());
        storage.auctions.insert(auction_id, Option::Some(auction));
        storage.deposits.insert((sender, auction_id), Option::None()); 

        let bought_asset = Asset{
            amount: reserve.unwrap(),
            contract_id: auction.buy_asset.contract_id,
            nft_id: auction.buy_asset.nft_id
        };
        log(WithdrawEvent{asset: bought_asset, auction_id: auction_id, identity: sender});  
        true
    }

    /// Starts a auction with the seller, selling asset, buying asset, 
    /// prices, and length of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The transaction did not have any sell asset
    /// - The inital price is higher than the reserve price if a reserve price is set
    /// - The time for the auction to end is 0
    /// - The token amount tranfered is not the amount specified in sell_asset
    /// - The token contract transfered is not the type specified in sell_asset
    /// - The auction contract is not approved to transfer all NFTs
    /// - The auction contract is not approved to transfer the specified token id in sell_asset
    /// - The auction contract is not the owner of the specified NFT
    /// - The sender is not approved to transfer all NFTs
    /// - The sender is not approved to transfer the specified token id in sell_asset
    /// - The sender is not the owner of the specified NFT
    #[storage(read, write)]
    fn constructor(seller: Identity, sell_asset: Asset, buy_asset: Asset, inital_price: u64, reserve_price: u64, time: u64) -> u64 {
        require(msg_amount() > 0, InputError::IncorrectAmountProvided);
        require((reserve_price >= inital_price && reserve_price != 0) || reserve_price == 0, InitError::ReserveLessThanInitalPrice);
        require(time != 0, InitError::AuctionTimeNotProvided);

        let nft_id: Option<u64> = sell_asset.nft_id;
        if (nft_id.is_none())
        {
            // Selling tokens
            require(msg_amount() == sell_asset.amount, InputError::IncorrectAmountProvided);
            require(msg_asset_id() == sell_asset.contract_id, InputError::IncorrectAssetProvided);
        } else {
            // Selling NFTs
            // Ensure that this contract is approved to transfer the token
            require(
                approved_for_nft_transfer(
                    Identity::ContractId(contract_id()), 
                    seller, 
                    sell_asset.contract_id, 
                    nft_id.unwrap()
                ), 
                AccessError::NFTTransferNotApproved
            );
            // Ensure that the sender is approved to transfer the token or is the owner
            let sender = sender_identity();
            require(
                approved_for_nft_transfer(
                    sender, 
                    seller, 
                    sell_asset.contract_id, 
                    nft_id.unwrap()
                ), 
                AccessError::NFTTransferNotApproved
            );
        }

        let reserve = match reserve_price {
            0 => Option::None(),
            _ => Option::Some(reserve_price),
        };

        let auction = Auction {
            buy_asset: buy_asset,
            bidder: Option::None(),
            end_block: height() + time,
            inital_price: inital_price,
            reserve_price: reserve,
            sell_asset: sell_asset,
            seller: seller,
            state: 1
        };

        storage.deposits.insert((seller, storage.total_auctions), Option::Some(sell_asset));
        storage.auctions.insert(storage.total_auctions, Option::Some(auction));
        log(AuctionStartEvent{auction: auction, auction_id: storage.total_auctions});

        storage.total_auctions = storage.total_auctions + 1;
        storage.total_auctions - 1
    }

    /// Returns the current bid of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction exists
    #[storage(read)]
    fn current_bid(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.buy_asset.amount
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the balance of the Address's buy asset deposits
    // fn deposits(identity: Identity, auction_id: u64) -> Option<Asset> {
    //     storage.deposits.get((identity, auction_id))
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the current bidder of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    // fn highest_bidder(auction_id: u64) -> Option<Identity> {
    //     let auction: Option<Auction> = storage.auctions.get(auction_id);
    //     require(auction.is_some(), AccessError::AuctionDoesNotExist);
    //     let auction = auction.unwrap();
    //     auction.bidder
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the reserve price of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    // fn reserve(auction_id: u64) -> Option<u64> {
    //     let auction: Option<Auction> = storage.auctions.get(auction_id);
    //     require(auction.is_some(), AccessError::AuctionDoesNotExist);
    //     let auction = auction.unwrap();
    //     auction.reserve_price
    // }

    /// Returns the amount of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]
    fn sell_amount(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.sell_asset.amount
    }

    /// Returns the contract id of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]
    fn sell_asset(auction_id: u64) -> ContractId {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.sell_asset.contract_id
    }

    /// Returns the current state of the function
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]
    fn state(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.state
    }

    /// Withdraws after the end of the auction
    ///
    /// # Panics
    /// 
    /// The function will panic when:
    /// - The auction does not exist
    /// - The auction time is not over
    /// - The auction state is not over
    /// - The buyer is the sender and already withdrew
    /// - The seller is the sender and already withdrew
    /// - The sender is not the buyer or seller and has nothing to withdraw
    #[storage(read, write)]
    fn withdraw(auction_id: u64) -> bool {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        require(auction.state == 2 || height() >= auction.end_block, AccessError::AuctionIsNotClosed);

        // If time has run out set the contract state to 2
        if (height() >= auction.end_block && auction.state == 1)
        {
            auction.state = 2;
        }

        let sender = sender_identity();
        let bidder: Option<Identity> = auction.bidder;
        let sell_nft_id: Option<u64> = auction.sell_asset.nft_id;
        let buy_nft_id: Option<u64> = auction.buy_asset.nft_id;
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        // TODO: Change this
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(BASE_ASSET_ID), nft_id: Option::None()},
        };

        require(sender_deposit.amount != 0, UserError::UserHasAlreadyWithdrawn);
        storage.deposits.insert((sender, auction_id), Option::None());
            
        if (bidder.is_some() && identities_equal(bidder.unwrap(), sender)) {
            // The buyer is withdrawing
            match sell_nft_id {
                Option::Some(u64) => transfer_nft(auction.seller, sender, auction.sell_asset),
                Option::None(u64) => send_tokens(sender, auction.sell_asset),
            };
        } else if (identities_equal(auction.seller, sender)) {
            // The seller is withdrawing
            if (bidder.is_none() && sell_nft_id.is_none()) {
                // No one placed a bid, only tokens should be transfered back
                send_tokens(sender, auction.sell_asset);
            } else { 
                // The asset was sold
                match buy_nft_id {
                    Option::Some(u64) => transfer_nft(bidder.unwrap(), sender, auction.buy_asset),
                    Option::None(u64) => send_tokens(sender, auction.buy_asset),
                }
            }
        } else if (buy_nft_id.is_none()) {
            // Anyone with a failed bid is withdrawing
            send_tokens(sender, sender_deposit);
        };

        storage.auctions.insert(auction_id, Option::Some(auction));
        true
    }
}
