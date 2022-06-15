library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{Campaign, CampaignInfo, Pledge};

abi Fundraiser {
    fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64);
    fn cancel_campaign(id: u64);
    fn claim_pledges(id: u64);
    fn pledge(id: u64);
    fn unpledge(id: u64, amount: u64);
    fn total_campaigns() -> u64;
    fn campaign_info(id: u64) -> CampaignInfo;
    fn campaign_count() -> u64;
    fn campaign(campaign_history_index: u64) -> Campaign;
    fn pledge_count() -> u64;
    fn pledged(pledge_history_index: u64) -> Pledge;
}
