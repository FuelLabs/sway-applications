library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::Campaign;

abi Fundraiser {
    fn constructor(owner: Identity);
    fn create_campaign(author: Identity, asset: ContractId, target_amount: u64, deadline: u64);
    fn pledge(id: u64);
    fn unpledge(id: u64, amount: u64);
    fn claim(id: u64);
    fn cancel(id: u64);
    fn get_campaign(id: u64) -> Campaign;
    fn get_pledge(id: u64) -> u64;
}
