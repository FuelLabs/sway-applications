library data_structures;

use std::{
    contract_id::ContractId,
    identity::Identity,
};

pub struct AirdropData {
    admin: Identity,
    claim_remaining: u64,
    end_block: u64,
    merkleRoot: b256,
    state: State,
    token_contract: ContractId,
}

pub struct Claim {
    amount: u64,
    identity: Identity,
}

pub enum State {
    Closed: (),
    Open: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Open, State::Open) => true, (State::Closed, State::Closed) => true, _ => false, 
        }
    }
}
