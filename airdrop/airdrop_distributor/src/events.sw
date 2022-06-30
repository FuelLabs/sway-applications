library events;

use std::{
    contract_id::ContractId,
    identity::Identity
};

pub struct ClaimEvent {
    amount: u64,
    claim_id: u64,
    to: Identity,
    token: ContractId,
}
