library data_structures;

use std::{contract_id::ContractId, identity::Identity};
use core::ops::Eq;

pub struct AssetInfo {
    amount: u64,
    exists: bool,
}

pub struct Campaign {
    id: u64,
}

pub struct CampaignInfo {
    author: Identity,
    asset: ContractId,
    beneficiary: Identity,
    cancelled: bool,
    claimed: bool,
    deadline: u64,
    target_amount: u64,
    total_pledge: u64,
}

pub struct Pledge {
    amount: u64,
    id: u64,
}
