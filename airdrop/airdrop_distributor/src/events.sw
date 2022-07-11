library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ClaimEvent {
    amount: u64,
    to: Identity,
}

pub struct InitializeEvent {
    end_block: u64,
    merkleRoot: b256,
    token_contract: ContractId,
}
