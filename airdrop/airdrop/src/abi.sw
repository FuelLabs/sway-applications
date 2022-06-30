library abi;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

abi Airdrop {
    #[storage(read, write)]fn claim(to: Identity, amount: u64, bytes: Vec<b256>);
    #[storage(read, write)]fn constructor(token_contract: ContractId, merkleRoot: b256);
}

abi Token {
    fn mint_to(to: Identity, amount: u64);
}
