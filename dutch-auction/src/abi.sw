library abi;

use std::{
    address::Address,
    contract_id::ContractId,
};

abi DutchAuction {
    fn get_price() -> u64;
    fn set_beneficiary(new_beneficiary: Address);
    fn bid();
    fn setup_auction(startp: u64, endp: u64, startt: u64, endt: u64, asset: ContractId);
}