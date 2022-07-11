contract;

dep events;
dep errors;
dep interface;
dep utils;

use events::{ClaimEvent, InitializeEvent};
use errors::{AccessError, InitError, StateError, VerificationError};
use interface::AirdropDistributor;
use utils::verify_merkle_proof;
use std::{
    assert::require,
    block::height,
    contract_id::ContractId,
    hash::sha256,
    identity::Identity,
    logging::log,
    storage::StorageMap,
    token::mint_to,
    vec::Vec,
};

storage {
    claimed: StorageMap<Identity,
    bool> = StorageMap {
    },
    end_block: u64 = 0,
    merkleRoot: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
}

impl AirdropDistributor for Contract {
    /// This function will let users claim their airdrop
    ///
    /// # Reverts
    ///
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>) {
        // The claiming period must be open
        require(storage.end_block < height(), StateError::ClaimPeriodHasEnded);

        // Make sure the `to` `Identity` hasn't already claimed
        require(!storage.claimed.get(to), AccessError::UserAlreadyClaimed);

        // Verify valid leaf
        require(verify_merkle_proof(storage.merkleRoot, sha256((to, amount)), proof), VerificationError::MerkleProofFailed);

        // Update the airdrop information
        storage.claimed.insert(to, true);

        // Mints tokens to the Identity claiming
        mint_to(amount, to);

        // Event
        log(ClaimEvent {
            to, amount, 
        });
    }

    /// Starts an airdrop
    ///
    /// # Reverts
    ///
    /// * The constructor has already been called.
    /// * The `claim_time` is set to zero.
    #[storage(read, write)]fn constructor(merkleRoot: b256, claim_time: u64) {
        require(storage.end_block == 0, InitError::AlreadyInitalized);
        require(claim_time != 0, InitError::ClaimTimeCannotBeZero);

        storage.end_block = claim_time;
        storage.merkleRoot = merkleRoot;

        // Log Event
        log(InitializeEvent {
            end_block: claim_time, merkleRoot
        });
    }
}
