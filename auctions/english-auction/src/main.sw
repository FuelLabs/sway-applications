contract;

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use abi::{EnglishAuction, NFT};
use data_structures::*;
use errors::{AccessError, InitError, InputError, UserError};
use events::{AuctionStartEvent, BidEvent, WithdrawEvent};
use utils::{
    approved_for_nft_transfer,
    reserve_met,
    send_tokens,
    sender_identity,
    transfer_nft,
    validate_corrent_asset,
};

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::*,
    logging::log,
    option::*,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

storage {
    auctions: StorageMap<u64,
    Option<Auction>>, // TODO: Move deposits into the Auction struct when StorageMaps are
    //       supported inside structs
    deposits: StorageMap<(Identity,
    u64), Option<Asset>>, total_auctions: u64,
}

impl EnglishAuction for Contract {
    /// Returns the block at which the auction will end
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]fn auction_end_block(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        auction.unwrap().end_block
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
    /// - The bidder is the seller
    /// - The asset provided is not the buy asset
    /// - The auction is not approved for transfer
    /// - The asset amount provided is less than the inital price if there are no bids
    /// - The asset amount provided plus current deposit is less than or equal to the current bid
    #[storage(read, write)]fn bid(auction_id: u64, asset: Asset) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Make sure this ia a open auction
        require(auction.state == 1, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        // Ensure this is the correct asset in the transaction, the Asset struct has the
        // correct information, and if it's an NFT we can transfer it to the auction contract
        validate_corrent_asset(auction.buy_asset, asset);

        // Set some variables we will need
        let sender = sender_identity();
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let reserve: Option<u64> = auction.reserve_price;
        let total_bid_asset = match sender_deposit {
            Option::Some(Asset) => {
                asset + sender_deposit.unwrap()
            },
            Option::None(Asset) => {
                asset
            }
        };

        // The bidder cannot be the seller
        require(sender != auction.seller, UserError::BidderIsSeller);

        // Make sure this is greater than inital bid
        if (auction.buy_asset.amount() == 0) {
            require(total_bid_asset.amount() >= auction.inital_price, InputError::InitalPriceNotMet);
        }

        // Make sure this bid is more than the last
        require(total_bid_asset.amount() > auction.buy_asset.amount(), InputError::IncorrectAmountProvided);

        // Finally, make the bid
        if (reserve.is_none() || total_bid_asset.amount() < reserve.unwrap()) {
            // There is no reserve or it was not met
            match total_bid_asset {
                Asset::NFTAsset(total_bid_asset) => {
                    // We need to transfer ownership to the auction contract if they are
                    // bidding a NFT
                    transfer_nft(sender, Identity::ContractId(contract_id()), asset);
                }
                _ => {}
            }

            // Update the auction
            auction.bidder = Option::Some(sender);
            auction.buy_asset = total_bid_asset;
            storage.auctions.insert(auction_id, Option::Some(auction));
            storage.deposits.insert((sender, auction_id), Option::Some(auction.buy_asset));

            // Log the bid
            log(BidEvent {
                asset: auction.buy_asset, auction_id: auction_id, identity: sender
            });
        } else {
            // The reserve price was met
            let auction_copy = auction;
            auction.state = 2;
            auction.bidder = Option::Some(sender);
            // auction.buy_asset.amount = reserve.unwrap();
            storage.deposits.insert((sender, auction_id), Option::None());
            storage.auctions.insert(auction_id, Option::Some(auction));
            reserve_met(auction_copy, total_bid_asset.amount(), reserve.unwrap());

            // Log the purchase
            log(WithdrawEvent {
                asset: auction.sell_asset, auction_id: auction_id, identity: sender
            });
        }
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
    #[storage(read, write)]fn buy_reserve(auction_id: u64, asset: Asset) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // This auction has to be open to bid
        require(auction.state == 1, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        // Can't buy the reserve if it doesn't exist
        let reserve: Option<u64> = auction.reserve_price;
        require(reserve.is_some(), AccessError::NoReserveSet);

        // Set some variables we will need
        let sender = sender_identity();
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let total_bid_asset = match sender_deposit {
            Option::Some(Asset) => {
                asset + sender_deposit.unwrap()
            },
            Option::None(Asset) => {
                asset
            }
        };

        // Make sure the sender is not the seller
        require(sender != auction.seller, UserError::BidderIsSeller);

        // Ensure this is the correct asset in the transaction, the Asset struct has the
        // correct information, and if it's an NFT we can transfer it to the auction contract
        validate_corrent_asset(auction.buy_asset, asset);

        // Make sure this bid is greater than or equal to the reserve
        require(total_bid_asset.amount() >= reserve.unwrap(), InputError::IncorrectAmountProvided);

        // Now the reserve price was met and the sender can purchase at the reserve price
        // There is no reserve or it was not met
        match total_bid_asset {
            Asset::NFTAsset(total_bid_asset) => {
                // We need to transfer ownership to the auction contract if they are
                // bidding a NFT
                transfer_nft(sender, Identity::ContractId(contract_id()), asset);
            }
            _ => {}
        }

        // Transfer the assets and update the auction
        let auction_copy = auction;
        auction.state = 2;
        auction.bidder = Option::Some(sender);
        // auction.buy_asset.amount = reserve.unwrap();
        storage.deposits.insert((sender, auction_id), Option::None());
        storage.auctions.insert(auction_id, Option::Some(auction));
        reserve_met(auction, total_bid_asset.amount(), reserve.unwrap());

        // Log the purchase
        log(WithdrawEvent {
            asset: auction.sell_asset, auction_id: auction_id, identity: sender
        });
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
    #[storage(read, write)]fn constructor(seller: Identity, sell_asset: Asset, buy_asset: Asset, inital_price: u64, reserve_price: u64, time: u64) -> u64 {
        require(msg_amount() > 0, InputError::IncorrectAmountProvided);
        require((reserve_price >= inital_price && reserve_price != 0) || reserve_price == 0, InitError::ReserveLessThanInitalPrice);
        require(time != 0, InitError::AuctionTimeNotProvided);

        // If this is an NFT to be auctioned we don't have to worry about msg_amount
        match sell_asset {
            Asset::TokenAsset(asset) => {
                // Selling tokens
                require(msg_amount() == asset.amount, InputError::IncorrectAmountProvided);
                require(msg_asset_id() == asset.contract_id, InputError::IncorrectAssetProvided);
            },
            Asset::NFTAsset(asset) => {
                // Selling NFTs
                // Ensure that the sender is approved to transfer the token or is the owner
                let sender = sender_identity();
                require(approved_for_nft_transfer(sender, seller, asset.contract_id, asset.token_ids), AccessError::NFTTransferNotApproved);

                // Transfer NFT to this contract
                transfer_nft(seller, Identity::ContractId(contract_id()), sell_asset);
            }
        }

        // Does the seller want a reserve
        let reserve = match reserve_price {
            0 => Option::None(), _ => Option::Some(reserve_price), 
        };

        // Setup auction
        let auction = Auction {
            buy_asset: buy_asset,
            bidder: Option::None(),
            end_block: height() + time,
            inital_price: inital_price,
            reserve_price: reserve,
            sell_asset: sell_asset,
            seller: seller,
            state: 1,
        };

        storage.deposits.insert((seller, storage.total_auctions), Option::Some(sell_asset));
        storage.auctions.insert(storage.total_auctions, Option::Some(auction));

        // Log the start of the new auction
        log(AuctionStartEvent {
            auction: auction, auction_id: storage.total_auctions
        });

        // Return the auction ID and increment the total auctions counter
        storage.total_auctions = storage.total_auctions + 1;
        storage.total_auctions - 1
    }

    /// Returns the current bid of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction exists
    #[storage(read)]fn current_bid(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.buy_asset.amount()
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
    /// Returns the balance of the Address's buy asset deposits
    // #[storage(read)]
    // fn deposits(identity: Identity, auction_id: u64) -> Option<Asset> {
    //     storage.deposits.get((identity, auction_id))
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
    /// Returns the current bidder of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    // #[storage(read)]
    // fn highest_bidder(auction_id: u64) -> Option<Identity> {
    //     let auction: Option<Auction> = storage.auctions.get(auction_id);
    //     require(auction.is_some(), AccessError::AuctionDoesNotExist);
    //     auction.unwrap().bidder
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
    /// Returns the reserve price of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    // #[storage(read)]
    // fn reserve(auction_id: u64) -> Option<u64> {
    //     let auction: Option<Auction> = storage.auctions.get(auction_id);
    //     require(auction.is_some(), AccessError::AuctionDoesNotExist);
    //     auction.unwrap().reserve_price
    // }

    /// Returns the amount of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]fn sell_amount(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.sell_asset.amount()
    }

    /// Returns the contract id of asset that is being sold
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]fn sell_asset(auction_id: u64) -> ContractId {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();
        auction.sell_asset.contract_id()
    }

    /// Returns the current state of the function
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction does not exist
    #[storage(read)]fn state(auction_id: u64) -> u64 {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        auction.unwrap().state
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
    #[storage(read, write)]fn withdraw(auction_id: u64) {
        // Make sure this auction exists
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let mut auction = auction.unwrap();

        // Cannot withdraw if the auction is over
        require(auction.state == 2 || height() >= auction.end_block, AccessError::AuctionIsNotClosed);

        // If time has run out set the contract state to 2
        if (height() >= auction.end_block && auction.state == 1) {
            auction.state = 2;
            storage.auctions.insert(auction_id, Option::Some(auction));
        }

        // Set some variables we will need
        let sender = sender_identity();
        let bidder: Option<Identity> = auction.bidder;
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));

        // Make sure the sender has something to withdraw
        require(sender_deposit.is_some(), UserError::UserHasAlreadyWithdrawn);
        storage.deposits.insert((sender, auction_id), Option::None());

        // Go ahead and withdraw
        if (bidder.is_some() && sender == bidder.unwrap()) {
            // The buyer is withdrawing
            match sender_deposit.unwrap() {
                Asset::NFTAsset(asset) => {
                    transfer_nft(Identity::ContractId(contract_id()), sender, auction.sell_asset)
                },
                Asset::TokenAsset(asset) => {
                    send_tokens(sender, auction.sell_asset)
                },
            };
        } else if (sender == auction.seller) {
            // The seller is withdrawing
            if (bidder.is_none()) {
                // No one placed a bid
                match sender_deposit.unwrap() {
                    Asset::NFTAsset(asset) => {
                        transfer_nft(Identity::ContractId(contract_id()), auction.seller, auction.sell_asset)
                    },
                    Asset::TokenAsset(asset) => {
                        send_tokens(sender, auction.sell_asset)
                    },
                }
            } else {
                // The asset was sold
                match sender_deposit.unwrap() {
                    Asset::NFTAsset(asset) => {
                        transfer_nft(Identity::ContractId(contract_id()), sender, auction.buy_asset)
                    },
                    Asset::TokenAsset(asset) => {
                        send_tokens(sender, auction.buy_asset)
                    },
                }
            }
        } else {
            // Anyone with a failed bid is withdrawing
            match sender_deposit.unwrap() {
                Asset::NFTAsset(asset) => {
                    transfer_nft(Identity::ContractId(contract_id()), sender, sender_deposit.unwrap())
                },
                Asset::TokenAsset(asset) => {
                    send_tokens(sender, sender_deposit.unwrap())
                },
            }
        };
    }
}
