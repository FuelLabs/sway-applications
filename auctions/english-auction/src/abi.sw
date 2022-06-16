library abi;

dep data_structures;

use data_structures::Asset;
use::std::{identity::Identity, contract_id::ContractId, option::Option};

abi EnglishAuction {
    fn auction_end_block(auction_id: u64) -> u64;
    fn bid(auction_id: u64) -> bool;
    fn buy_reserve(auction_id: u64) -> bool;
    fn constructor(seller: Identity, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> u64;
    fn current_bid(auction_id: u64) -> u64;
    // fn deposits(identity: Identity, auction_id: u64) -> Option<Asset>;
    // fn highest_bidder(auction_id: u64) -> Option<Identity>;
    // fn reserve(auction_id: u64) -> Option<u64>;
    fn sell_amount(auction_id: u64) -> u64;
    fn sell_asset(auction_id: u64) -> ContractId;
    fn state(auction_id: u64) -> u64;
    fn withdraw(auction_id: u64) -> bool;
}
