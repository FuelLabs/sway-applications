library events;

dep data_structures;

use data_structures::AirdropData;
use std::{contract_id::ContractId, identity::Identity};

pub struct CreateEvent {
    airdrop: AirdropData,
    claim_id: u64,
}

pub struct ClaimEvent {
    amount: u64,
    claim_id: u64,
    to: Identity,
    token: ContractId,
}

pub struct ReClaimEvent {
    airdrop: AirdropData,
    claim_id: u64,
}
