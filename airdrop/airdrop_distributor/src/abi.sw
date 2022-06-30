library abi;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

abi AirdropDistributor {
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>, token: ContractId, claim_id: u64);
    #[storage(read, write)]fn constructor(token_contract: ContractId, merkleRoot: b256, admin: Identity, claim_time: u64) -> u64;
}

abi Token {
    fn mint_to(to: Identity, amount: u64);
}
