library entity;

use std::{
    address::Address,
    contract_id::ContractId,
};
use core::ops::Eq;

pub struct Entity {
    address: Address,
    contract_id: ContractId,
    // TODO: Add enum when supported in storage
    //       For now, Address: 1, Contract: 2
    //identity: Identity,
    identity: u64,
}

impl Eq for Entity {
    fn eq(self, other: Self) -> bool {
        self.address == other.address 
        && self.contract_id == other.contract_id 
        && self.identity == other.identity
    }
}