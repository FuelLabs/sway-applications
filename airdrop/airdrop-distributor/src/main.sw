contract;

dep events;
dep errors;
dep interface;
dep utils;

use events::{ClaimEvent, InitializeEvent};
use errors::{AccessError, InitError, StateError, VerificationError};
use interface::AirdropDistributor;
use utils::{mint_to, verify_merkle_proof};
use std::{
    assert::require,
    block::height,
    contract_id::ContractId,
    hash::sha256,
    identity::Identity,
    logging::log,
    option::Option,
    storage::StorageMap,
    vec::Vec,
};

storage {
    /// Stores true if a user has claimed their airdrop. Maps a tuple of a user and an amount to a
    /// boolean.
    /// Maps ((user, amount) => claim)
    claimed: StorageMap<(Identity,
    u64), bool> = StorageMap {
    },
    /// The block at which the claiming period will end.
    end_block: u64 = 0,
    /// The computer merkle root which is to be verified against.
    merkleRoot: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    /// The contract of the token which is to be distributed.
    token_contract: Option<ContractId> = Option::None,
}

impl AirdropDistributor for Contract {
    #[storage(read, write)]fn claim(amount: u64, proof: Vec<b256>, to: Identity) {
        // The claiming period must be open and the `to` identity hasn't already claimed
        require(storage.end_block < height(), StateError::ClaimPeriodHasEnded);
        require(!storage.claimed.get((to, amount)), AccessError::UserAlreadyClaimed);

        // Verify the merkle proof against the user and amount
        require(verify_merkle_proof(sha256((to, amount)), storage.merkleRoot, proof), VerificationError::MerkleProofFailed);

        // Mint tokens
        storage.claimed.insert((to, amount), true);
        mint_to(amount, to, storage.token_contract.unwrap());

        log(ClaimEvent {
            to, amount, 
        });
    }

    #[storage(read, write)]fn constructor(claim_time: u64, merkleRoot: b256, token_contract: ContractId) {
        // If `end_block` is set to a value other than 0, we know that the contructor has already
        // been called.
        require(storage.end_block == 0, InitError::AlreadyInitialized);
        require(claim_time != 0, InitError::ClaimTimeCannotBeZero);

        storage.end_block = height() + claim_time;
        storage.merkleRoot = merkleRoot;
        storage.token_contract = Option::Some(token_contract);

        log(InitializeEvent {
            end_block: claim_time, merkleRoot, token_contract
        });
    }
}
