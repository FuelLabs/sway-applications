library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{Campaigns, Campaign, Pledge};

abi Fundraiser {
    fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64);
    fn pledge(id: u64);
    fn unpledge(id: u64, amount: u64);
    fn claim(id: u64);
    fn cancel(id: u64);
    fn campaign_count() -> u64;
    fn campaign_info(id: u64) -> Campaign;
    fn pledge_count() -> u64;
    fn pledged(id: u64) -> Pledge;
    fn campaigns() -> Campaigns;
    fn update_campaign_state(id: u64);
}
