library utils;

dep abi;

use abi::Token;
use std::{
    contract::ContractId,
    hash::sha256,
};

/// This function returns the hash of a claim struct
fn create_hash(identity: Identity, amount: u64) -> b256 {
    sha256(Claim {
        identity, amount
    })
}

/// Calls a token contract and mints to an address
fn mint(token: ContractId, to: Identity, amount: u64) {
    let token_abi = abi(Token, token);
    token_abi.mint_to(to, amount);
}
