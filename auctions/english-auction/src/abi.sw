library abi;

use::std::{identity::Identity, contract_id::ContractId};

abi EnglishAuction {
    fn auction_end_block() -> u64;
    fn bid() -> bool;
    fn buy_reserve() -> bool;
    fn constructor(seller: Identity, buy_asset: ContractId, inital_price: u64, reserve_price: u64, time: u64) -> bool;
    fn current_bid() -> u64;
    fn deposits(identity: Identity) -> u64;
    // fn highest_bidder() -> Option<Identity>;
    fn reserve() -> u64;
    fn sell_amount() -> u64;
    fn sell_asset() -> ContractId;
    fn state() -> u64;
    fn withdraw() -> bool;
}