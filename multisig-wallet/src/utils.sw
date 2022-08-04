library utils;

dep data_structures;

use data_structures::Transaction;
use std::{contract_id::ContractId, hash::sha256, identity::Identity};

pub fn create_hash(to: Identity, value: u64, data: [u64; 3], nonce: u64, self_id: ContractId) -> b256 {
    sha256(Transaction {
        contract_identifier: self_id, destination: to, value, data, nonce
    })
}
