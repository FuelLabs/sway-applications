library abi;

dep data_structures;

use std::{chain::auth::Sender, contract_id::ContractId};
use data_structures::Campaign;

abi Fundraiser {
    fn constructor(owner: Sender);
    fn create_campaign(author: Sender, asset: ContractId, target_amount: u64, deadline: u64);
    fn pledge(campaign_identifier: u64);
    fn unpledge(campaign_identifier: u64, amount: u64);
    fn claim(campaign_identifier: u64);
    fn cancel(campaign_identifier: u64);
    fn get_campaign(campaign_identifier: u64) -> Campaign;
    fn get_pledge(campaign_identifier: u64) -> u64;
}
