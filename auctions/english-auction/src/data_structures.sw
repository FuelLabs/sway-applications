library data_structures;

use std::{
    contract_id::ContractId, 
    identity::Identity, 
    option::Option, 
    storage::StorageMap
};

pub struct Asset {
    amount: u64,
    contract_id: ContractId,
}

pub struct Auction {
    buy_asset: Asset,
    bidder: Option<Identity>,
    end_block: u64,
    inital_price: u64,
    reserve_price: Option<u64>,
    sell_asset: Asset,
    seller: Identity,
    state: u64,
}
