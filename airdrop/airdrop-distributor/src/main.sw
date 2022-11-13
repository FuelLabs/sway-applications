contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::ClaimData;
use errors::{AccessError, InitError, StateError, VerificationError};
use events::{ClaimEvent, CreateAirdropEvent};
use interface::AirdropDistributor;
use std::{block::height, hash::sha256, logging::log, storage::StorageMap};
use sway_libs::binary_merkle_proof::{leaf_digest, verify_proof};
use utils::mint_to;

storage {
    /// The contract of the asset which is to be distributed.
    asset: Option<ContractId> = Option::None,
    /// Stores the ClaimData struct of users that have interacted with the Airdrop Distrubutor contract.
    /// Maps (user => claim)
    claims: StorageMap<Identity, ClaimData> = StorageMap {},
    /// The block at which the claiming period will end.
    end_block: u64 = 0,
    /// The computed merkle root which is to be verified against.
    merkle_root: Option<b256> = Option::None,
}

impl AirdropDistributor for Contract {
    #[storage(read, write)]
    fn claim(
        amount: u64,
        key: u64,
        num_leaves: u64,
        proof: Vec<b256>,
        to: Identity,
    ) {
        // The claiming period must be open and the `to` identity hasn't already claimed
        require(storage.end_block > height(), StateError::ClaimPeriodHasEnded);
        require(!storage.claims.get(to).claimed, AccessError::UserAlreadyClaimed);

        // Verify the merkle proof against the user and amount
        let leaf = leaf_digest(sha256((to, amount)));
        require(verify_proof(key, leaf, storage.merkle_root.unwrap(), num_leaves, proof), VerificationError::MerkleProofFailed);

        // Mint asset
        storage.claims.insert(to, ~ClaimData::new(amount, true));
        mint_to(amount, storage.asset.unwrap(), to);

        log(ClaimEvent { to, amount });
    }

    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimData {
        storage.claims.get(identity)
    }

    #[storage(read, write)]
    fn constructor(asset: ContractId, claim_time: u64, merkle_root: b256) {
        // If `end_block` is set to a value other than 0, we know that the contructor has already
        // been called.
        require(storage.end_block == 0, InitError::AlreadyInitialized);

        storage.end_block = height() + claim_time;
        storage.merkle_root = Option::Some(merkle_root);
        storage.asset = Option::Some(asset);

        log(CreateAirdropEvent {
            asset,
            end_block: claim_time,
            merkle_root,
        });
    }

    #[storage(read)]
    fn end_block() -> u64 {
        storage.end_block
    }

    #[storage(read)]
    fn merkle_root() -> b256 {
        require(storage.merkle_root.is_some(), InitError::NotInitalized);
        storage.merkle_root.unwrap()
    }
}
