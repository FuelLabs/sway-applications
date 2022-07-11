library utils;

dep interface;

use interface::Token;
use std::{contract_id::ContractId, hash::sha256, identity::Identity, option::Option, revert::revert, vec::Vec};

pub fn mint_to(amount: u64, to: Identity, token: ContractId) {
    let token_abi = abi(Token, token.value);
    token_abi.mint_to(amount, to);
}

pub fn verify_merkle_proof(merkleLeaf: b256, merkleRoot: b256, proof: Vec<b256>) -> bool {
    let mut computedHash = merkleLeaf;
    let mut index = 0;

    // Itterate over proof
    while index < proof.len() {
        // Get the current element in the proof
        let proofElement: Option<b256> = proof.get(index);
        let proofElement = match proofElement {
            Option::Some(b256) => proofElement.unwrap(), Option::None(b256) => revert(0), 
        };

        if (computedHash <= proofElement) {
            // Hash(current computed hash + current element of the proof)
            computedHash = sha256((computedHash, proofElement));
        } else {
            // Hash(current element of the proof + current computed hash)
            computedHash = sha256((proofElement, computedHash));
        }

        index = index + 1;
    }

    // Check if the computed hash (root) is equal to the provided root
    computedHash == merkleRoot
}
