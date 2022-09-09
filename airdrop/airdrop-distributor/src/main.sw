contract;

dep events;
dep errors;
dep data_structures;
dep interface;
dep utils;

use events::{ClaimEvent, CreateAirdropEvent};
use errors::{AccessError, InitError, StateError, VerificationError};
use data_structures::ClaimData;
use interface::AirdropDistributor;
use std::{
    address::Address,
    block::height,
    contract_id::ContractId,
    hash::sha256,
    identity::Identity,
    logging::log,
    revert::require,
    storage::StorageMap,
};
use sway_libs::binary_merkle_proof::{leaf_digest, verify_proof};
use utils::mint_to;

storage {
    /// Stores the ClaimData struct of users that have interacted with the Airdrop Distrubutor contract.
    /// Maps (user => claim)
    claims: StorageMap<Identity, ClaimData> = StorageMap {},
    /// The block at which the claiming period will end.
    end_block: u64 = 0,
    /// The computed merkle root which is to be verified against.
    merkle_root: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    /// The contract of the asset which is to be distributed.
    asset: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

impl AirdropDistributor for Contract {
    #[storage(read, write)]
    fn claim(
        amount: u64,
        key: u64,
        num_leaves: u64,
        proof: [b256; 2],
        to: Identity,
    ) {
        // The claiming period must be open and the `to` identity hasn't already claimed
        require(storage.end_block > height(), StateError::ClaimPeriodHasEnded);
        require(!storage.claims.get(to).claimed, AccessError::UserAlreadyClaimed);

        // Verify the merkle proof against the user and amount
        let leaf = leaf_digest(sha256((to, amount)));
        require(verify_proof(key, leaf, storage.merkle_root, num_leaves, proof), VerificationError::MerkleProofFailed);

        // Mint asset
        storage.claims.insert(to, ~ClaimData::new(amount, true));
        mint_to(amount, to, storage.asset);

        log(ClaimEvent {
            to,
            amount,
        });
    }

    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimData {
        storage.claims.get(identity)
    }

    #[storage(read, write)]
    fn constructor(claim_time: u64, merkle_root: b256, asset: ContractId, ) {
        // If `end_block` is set to a value other than 0, we know that the contructor has already
        // been called.
        require(storage.end_block == 0, InitError::AlreadyInitialized);

        storage.end_block = height() + claim_time;
        storage.merkle_root = merkle_root;
        storage.asset = asset;

        log(CreateAirdropEvent {
            end_block: claim_time,
            merkle_root,
            asset,
        });
    }

    #[storage(read)]
    fn end_block() -> u64 {
        storage.end_block
    }

    #[storage(read)]
    fn merkle_root() -> b256 {
        storage.merkle_root
    }
}
