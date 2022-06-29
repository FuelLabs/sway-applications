library abi;

use std::{
    contract::ContractId,
    identity::Identity,
};

abi Airdrop {
    fn claim(to: Identity, amount: u64, bytes: b256);
    fn constructor(token_contract: ContractId);
}

abi Token {
    fn mint_to(to: Identity, amount: u64);
}
