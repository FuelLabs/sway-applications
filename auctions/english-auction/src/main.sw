contract;

dep abi;
dep data_structures;
dep errors;

use abi::EnglishAuction;
use data_structures::{Asset, Auction};
use errors::{AccessError, InitError, InputError, UserError};

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
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

        let sender: Identity = unwrap_identity(msg_sender());
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(NATIVE_ASSET_ID)},
        };
        
        require(!compare_identities(sender, auction.seller), UserError::BidderIsSeller);
        require(msg_amount() + sender_deposit.amount >= current_bid, InputError::IncorrectAmountProvided);

        let reserve: Option<u64> = auction.reserve_price;
        if (reserve.is_none() || msg_amount() + sender_deposit.amount < reserve.unwrap()) {
            // If the reserve price has not yet been met
            auction.bidder = Option::Some(sender);
            auction.buy_asset.amount = sender_deposit.amount + msg_amount();
            storage.auctions.insert(auction_id, Option::Some(auction));
            storage.deposits.insert((sender, auction_id), Option::Some(auction.buy_asset));
        } else {
            // The reserve price was met
            reserve_met(sender, sender_deposit.amount, auction_id, reserve.unwrap());
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
    fn buy_reserve(auction_id: u64) -> bool {
        let auction: Option<Auction> = storage.auctions.get(auction_id);
        require(auction.is_some(), AccessError::AuctionDoesNotExist);
        let auction = auction.unwrap();

        require(auction.state == 1, AccessError::AuctionIsNotOpen);
        require(height() <= auction.end_block, AccessError::AuctionIsNotOpen);

        let reserve: Option<u64> = auction.reserve_price;
        require(reserve.is_some(), AccessError::NoReserveSet);

        let sender = unwrap_identity(msg_sender());
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(NATIVE_ASSET_ID)},
        };

        require(!compare_identities(sender, auction.seller), UserError::BidderIsSeller);
        require(msg_amount() + sender_deposit.amount >= reserve.unwrap(), InputError::IncorrectAmountProvided);
        require(msg_asset_id() == auction.buy_asset.contract_id, InputError::IncorrectAssetProvided);

        reserve_met(sender, sender_deposit.amount, auction_id, reserve.unwrap());
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
    fn constructor(seller: Identity, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> u64 {
        require(msg_amount() > 0, InputError::IncorrectAmountProvided);
        require((reserve_price >= inital_price && reserve_price != 0) || reserve_price == 0, InitError::ReserveLessThanInitalPrice);
        require(time != 0, InitError::AuctionTimeNotProvided);

        let buy_asset = Asset {
            amount: 0,
            contract_id: buy_asset
        };

        let sell_asset = Asset {
            amount: msg_amount(),
            contract_id: msg_asset_id()
        };

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
        storage.total_auctions = storage.total_auctions + 1;
        storage.total_auctions - 1
    }

    /// Returns the current bid of the auction
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The auction exists
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

        let sender = unwrap_identity(msg_sender());
        let sender_deposit: Option<Asset> = storage.deposits.get((sender, auction_id));
        let sender_deposit = match sender_deposit {
            Option::Some(Asset) => sender_deposit.unwrap(),
            Option::None(Asset) => Asset {amount: 0, contract_id: ~ContractId::from(NATIVE_ASSET_ID)},
        };

        require(sender_deposit.amount != 0, UserError::UserHasAlreadyWithdrawn);
        storage.deposits.insert((sender, auction_id), Option::None());
            
        let bidder: Option<Identity> = auction.bidder;
        if (bidder.is_none() && compare_identities(bidder.unwrap(), sender)) {
            // The buyer is withdrawing
            match sender {
                Identity::Address(sender) => {
                    transfer_to_output(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);    
                },
                Identity::ContractId(sender) => {
                    force_transfer_to_contract(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);
                },
            };
        } else if (compare_identities(auction.seller, sender)) {
            // The seller is withdrawing
            if (bidder.is_none()) {
                // No one placed a bid
                match sender {
                    Identity::Address(sender) => {
                        transfer_to_output(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);    
                    },
                    Identity::ContractId(sender) => {
                        force_transfer_to_contract(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);
                    },
                };
            } else { 
                // The asset was sold
                match sender {
                    Identity::Address(sender) => {
                        transfer_to_output(auction.buy_asset.amount, auction.buy_asset.contract_id, sender);    
                    },
                    Identity::ContractId(sender) => {
                        force_transfer_to_contract(auction.buy_asset.amount, auction.buy_asset.contract_id, sender);
                    },
                };
            }
        } else {
            // Anyone with a failed bid is withdrawing
            match sender {
                Identity::Address(sender) => {
                    transfer_to_output(sender_deposit.amount, auction.buy_asset.contract_id, sender);    
                },
                Identity::ContractId(sender) => {
                    force_transfer_to_contract(sender_deposit.amount, auction.buy_asset.contract_id, sender);
                },
            };
        };

        storage.auctions.insert(auction_id, Option::Some(auction));
        true
    }
}

// This function will take two identities and return true if they are the same
fn compare_identities(identity1: Identity, identity2: Identity) -> bool {
    match identity1 {
        Identity::Address(identity1) => {
            match identity2 {
                Identity::Address(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        },
        Identity::ContractId(identity1) => {
            match identity2 {
                Identity::ContractId(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        }
    }
}


// Gets called when the reserve price is met
fn reserve_met(sender: Identity, balance: u64, auction_id: u64, reserve: u64) {
    let auction: Option<Auction> = storage.auctions.get(auction_id);
    let mut auction: Auction = auction.unwrap();
    auction.state = 2;
    auction.bidder = Option::Some(sender);
    auction.buy_asset.amount = reserve;
    storage.deposits.insert((sender, auction_id), Option::None());

    match sender {
        Identity::Address(sender) => {
            transfer_to_output(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);    
        },
        Identity::ContractId(sender) => {
            force_transfer_to_contract(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);
        },
    };

    let overpaid_balance = (msg_amount() + balance) - reserve;
    if (overpaid_balance > 0)
    {
        match sender {
            Identity::Address(sender) => {
                transfer_to_output(overpaid_balance, auction.buy_asset.contract_id, sender);    
            },
            Identity::ContractId(sender) => {
                force_transfer_to_contract(overpaid_balance, auction.buy_asset.contract_id, sender);
            },
        };
    }

    storage.auctions.insert(auction_id, Option::Some(auction));
}

fn unwrap_identity(sender: Result<Identity, AuthError>) -> Identity {
    sender.unwrap()
}
