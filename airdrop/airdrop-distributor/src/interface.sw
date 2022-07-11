library interface;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

abi AirdropDistributor {
    #[storage(read, write)]fn claim(amount: u64, proof: Vec<b256>, to: Identity);
    #[storage(read, write)]fn constructor(claim_time: u64, merkleRoot: b256, token: ContractId);
}

abi Token {
    fn mint_to(amount: u64, to: Identity);
}
