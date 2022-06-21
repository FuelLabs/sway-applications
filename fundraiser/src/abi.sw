library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{AssetInfo, Campaign, CampaignInfo, Pledge};

abi Fundraiser {
    #[storage(read, write)]fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64);

    #[storage(read, write)]fn cancel_campaign(id: u64);

    #[storage(read, write)]fn claim_pledges(id: u64);

    #[storage(read, write)]fn pledge(id: u64);

    #[storage(read, write)]fn unpledge(id: u64, amount: u64);

    #[storage(read)]fn total_campaigns() -> u64;

    #[storage(read)]fn campaign_info(id: u64) -> CampaignInfo;

    #[storage(read)]fn campaign_count() -> u64;

    #[storage(read)]fn campaign(campaign_history_index: u64) -> Campaign;

    #[storage(read)]fn pledge_count() -> u64;

    #[storage(read)]fn pledged(pledge_history_index: u64) -> Pledge;

    #[storage(read)]fn asset_count() -> u64;

    #[storage(read)]fn asset_info_by_address(asset: ContractId) -> AssetInfo;

    #[storage(read)]fn asset_info_by_count(index: u64) -> AssetInfo;
}
