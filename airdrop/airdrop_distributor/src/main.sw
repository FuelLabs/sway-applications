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
    claimed: StorageMap<Identity,
    bool> = StorageMap {
    },
    end_block: u64 = 0,
    merkleRoot: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    token_contract: Option<ContractId> = Option::None,
}

impl AirdropDistributor for Contract {
    /// This function will let users claim their airdrop.
    ///
    /// # Reverts
    ///
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>) {
        // The claiming period must be open and the `to` identity hasn't already claimed
        require(storage.end_block < height(), StateError::ClaimPeriodHasEnded);
        require(!storage.claimed.get(to), AccessError::UserAlreadyClaimed);

        // Verify valid leaf
        require(verify_merkle_proof(storage.merkleRoot, sha256((to, amount)), proof), VerificationError::MerkleProofFailed);

        // Mint tokens
        storage.claimed.insert(to, true);
        mint_to(amount, to, storage.token_contract.unwrap());

        // Event
        log(ClaimEvent {
            to, amount, 
        });
    }

    /// Starts an airdrop.
    ///
    /// # Reverts
    ///
    /// * The constructor has already been called.
    /// * The `claim_time` is set to zero.
    #[storage(read, write)]fn constructor(merkleRoot: b256, claim_time: u64, token_contract: ContractId) {
        require(storage.end_block == 0, InitError::AlreadyInitalized);
        require(claim_time != 0, InitError::ClaimTimeCannotBeZero);

        storage.end_block = claim_time;
        storage.merkleRoot = merkleRoot;
        storage.token_contract = Option::Some(token_contract);

        // Log Event
        log(InitializeEvent {
            end_block: claim_time, merkleRoot, token_contract
        });
    }
}
