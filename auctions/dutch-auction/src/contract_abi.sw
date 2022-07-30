library contract_abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::Auction;

abi DutchAuction {
    #[storage(read)]fn active_auctions_of_author(identity_to_check: Identity) -> [u64;
    1];
    #[storage(read)]fn auction(auction_id: u64) -> Auction;
    #[storage(read)]fn auctions_of_author(author: Identity) -> [u64;
    1];
    #[storage(read)]fn auctions_won(bidder: Identity) -> [u64;
    1];
    #[storage(read, write)]fn bid(auction_id: u64);
    #[storage(read, write)]fn cancel_auction(auction_id: u64);
    #[storage(read, write)]fn change_asset(new_asset: ContractId, auction_id: u64);
    #[storage(read, write)]fn change_beneficiary(new_beneficiary: Identity, auction_id: u64);
    #[storage(read, write)]fn create_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64, beneficiary: Identity, asset: ContractId);
    #[storage(read)]fn price(auction_id: u64) -> u64;
}
