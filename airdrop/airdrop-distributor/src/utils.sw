library utils;

dep interface;

use interface::SimpleToken;
use std::{
    contract_id::ContractId,
    hash::sha256,
    identity::Identity,
    option::Option,
    revert::revert,
    vec::Vec,
};

/// Calls the `mint_to` function in another contract.
///
/// # Arguments
///
/// `amount` - The quantity of tokens to be minted.
/// `to` - The user which the tokens should be given to.
/// `token` - The external token contract which has an implemented `mint_to` function.
pub fn mint_to(amount: u64, to: Identity, token: ContractId) {
    let token_abi = abi(SimpleToken, token.value);
    token_abi.mint_to(amount, to);
}

/// This function will verify the merkle leaf and root given against the proof.
///
/// # Arguments
///
/// `merkle_leaf` - A hash of a (user, amount) tuple representing the allotted tokens to a user.
/// `merkle_root` - The computed merkle root given at the initialization of the airdrop contract.
/// `proof` - The merkle proof that will be used to verify the leaf and root hashes.
pub fn verify_merkle_proof(merkle_leaf: b256, merkle_root: b256, proof: Vec<b256>) -> bool {
    let mut computed_hash = merkle_leaf;
    let mut index = 0;
    let proof_length = proof.len();

    // Itterate over proof
    while index < proof_length {
        // Get the current element in the proof
        let proof_element: Option<b256> = proof.get(index);
        let proof_element = match proof_element {
            Option::Some(b256) => proof_element.unwrap(), Option::None(b256) => revert(0), 
        };

        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash = sha256((computed_hash, proof_element));
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash = sha256((proof_element, computed_hash));
        }

        index = index + 1;
    }

    // Check if the computed hash is equal to the provided root
    computed_hash == merkle_root
}
