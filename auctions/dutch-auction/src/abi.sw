library abi;

use std::{
    address::Address,
    contract_id::ContractId,
};

abi DutchAuction {
    fn get_price(auction_id: u64) -> u64;
    fn set_beneficiary(new_beneficiary: Address);
    fn bid();
    fn setup_auction(opening_price: u64, reserve_price: u64, start_time: u64, end_time: u64) -> u64;
    fn end_auction();
    fn constructor(admin: Address, asset: ContractId);
}