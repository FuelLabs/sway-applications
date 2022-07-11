library interface;

use std::{identity::Identity, vec::Vec};

abi AirdropDistributor {
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>);
    #[storage(read, write)]fn constructor(merkleRoot: b256, claim_time: u64);
}
