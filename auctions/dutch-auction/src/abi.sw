library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::Auction;

abi DutchAuction {
    fn price(auction_id: u64) -> u64;
    fn bid(auction_id: u64);
    fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Identity, asset: ContractId);
    fn cancel_auction(auction_id: u64);
    fn constructor(admin: Identity);
    fn auction(auction_id: u64) -> Auction;
}
