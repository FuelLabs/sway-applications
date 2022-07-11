library interface;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

abi AirdropDistributor {
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>);
    #[storage(read, write)]fn constructor(merkleRoot: b256, claim_time: u64, token: ContractId);
}

abi Token {
    fn mint_to(amount: u64, to: Identity);
}
