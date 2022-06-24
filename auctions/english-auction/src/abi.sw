library abi;

dep data_structures;

use data_structures::{Asset, Auction};
use std::{contract_id::ContractId, identity::Identity, option::Option};

abi EnglishAuction {
    // #[storage(read)]fn auction_info(auction_id: u64) -> Option<Auction>;
    #[storage(read, write)]fn bid(auction_id: u64, asset: Asset);
    #[storage(read, write)]fn cancel_auction(auction_id: u64);
    #[storage(read, write)]fn constructor(seller: Identity, sell_asset: Asset, buy_asset: Asset, inital_price: u64, reserve_price: u64, time: u64) -> u64;
    // #[storage(read)]fn deposits(identity: Identity, auction_id: u64) -> Option<Asset>;
    #[storage(read, write)]fn withdraw(auction_id: u64);
    #[storage(read)]fn total_auctions() -> u64;
}

abi NFT {
    fn get_approved(token_id: u64) -> Option<Identity>;
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    fn owner_of(token_id: u64) -> Option<Identity>;
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}
