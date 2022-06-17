library abi;

dep data_structures;

use data_structures::Asset;
use::std::{identity::Identity, contract_id::ContractId, option::Option};

abi EnglishAuction {
    #[storage(read)]
    fn auction_end_block(auction_id: u64) -> u64;
    #[storage(read, write)]
    fn bid(auction_id: u64, asset: Asset) -> bool;
    #[storage(read, write)]
    fn buy_reserve(auction_id: u64, asset: Asset) -> bool;
    #[storage(read, write)]
    fn constructor(seller: Identity, sell_asset: Asset, buy_asset: Asset, inital_price: u64, reserve_price: u64, time: u64) -> u64;
    #[storage(read)]
    fn current_bid(auction_id: u64) -> u64;
    // #[storage(read)]
    // fn deposits(identity: Identity, auction_id: u64) -> Option<Asset>;
    // #[storage(read)]
    // fn highest_bidder(auction_id: u64) -> Option<Identity>;
    // #[storage(read)]
    // fn reserve(auction_id: u64) -> Option<u64>;
    #[storage(read)]
    fn sell_amount(auction_id: u64) -> u64;
    #[storage(read)]
    fn sell_asset(auction_id: u64) -> ContractId;
    #[storage(read)]
    fn state(auction_id: u64) -> u64;
    #[storage(read, write)]
    fn withdraw(auction_id: u64) -> bool;
}

abi NFT {
    fn get_approved(token_id: u64) -> Option<Identity>;
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    fn owner_of(token_id: u64) -> Option<Identity>;
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}