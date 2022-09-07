library data_structures;

use std::{
    contract_id::ContractId, 
    identity::Identity,
};

/// Used to track the total amount pledged to an asset
pub struct AssetInfo {
    /// The amount that is currently pledged
    amount: u64,
    /// Given that an asset can be set in a campaign and never pledge to we this field to handle
    /// control flow
    exists: bool,
}

/// Used to track the campaigns that a user has created
pub struct Campaign {
    /// The unique identifier for the campaign
    id: u64,
}

/// General data structure containing information about a campaign
pub struct CampaignInfo {
    /// The user who has created the campaign
    author: Identity,
    /// The asset that this campaign accepts as a deposit
    asset: ContractId,
    /// The user to whom the funds will be sent to upon a successful campaign
    beneficiary: Identity,
    /// Whether the campaign has been cancelled by the author
    cancelled: bool,
    /// Whether the campaign has been claimed by the author
    claimed: bool,
    /// The end time for the campaign after which it becomes locked
    deadline: u64,
    /// The amount needed to deem the campaign a success
    target_amount: u64,
    /// The current amount pledged used to measure against the target_amount
    total_pledge: u64,
}

/// Used to track the amount pledged by a user to a specific campaign
pub struct Pledge {
    /// The amount pledged to a campaign
    amount: u64,
    /// The unique identifier for the campaign
    id: u64,
}
