contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::ClaimState;
use errors::{AccessError, InitError, StateError, VerificationError};
use events::{ClaimEvent, CreateAirdropEvent};
use interface::{AirdropDistributor, Info};
use merkle_proof::binary_merkle_proof::{leaf_digest, verify_proof};
use std::{block::height, hash::sha256};
use utils::mint_to;

storage {
    /// The contract of the asset which is to be distributed.
    asset: Option<ContractId> = Option::None,
    /// Stores the ClaimState struct of users that have interacted with the Airdrop Distrubutor contract.
    /// Maps (user => claim)
    claims: StorageMap<Identity, ClaimState> = StorageMap {},
    /// The block at which the claiming period will end.
    end_block: u64 = 0,
    /// The computed merkle root which is to be verified against.
    merkle_root: Option<b256> = Option::None,
    /// The number of leaves in the merkle tree
    num_leaves: u64 = 0,
}

impl AirdropDistributor for Contract {
    #[storage(read, write)]
    fn claim(amount: u64, key: u64, proof: Vec<b256>, to: Identity) {
        // The claiming period must be open and the `to` identity hasn't already claimed
        require(storage.end_block > height(), StateError::ClaimPeriodHasEnded);
        require(storage.claims.get(to).unwrap_or(ClaimState::Unclaimed) == ClaimState::Unclaimed, AccessError::UserAlreadyClaimed);

        // Verify the merkle proof against the user and amount
        let leaf = leaf_digest(sha256((to, amount)));
        require(verify_proof(key, leaf, storage.merkle_root.unwrap(), storage.num_leaves, proof), VerificationError::MerkleProofFailed);

        storage.claims.insert(to, ClaimState::Claimed(amount));

        // Mint asset
        mint_to(amount, storage.asset.unwrap(), to);

        log(ClaimEvent { to, amount });
    }

    #[storage(read, write)]
    fn clawback() {}

    #[storage(read, write)]
    fn constructor(
        asset: ContractId,
        claim_time: u64,
        merkle_root: b256,
        num_leaves: u64,
    ) {
        // If `end_block` is set to a value other than 0, we know that the contructor has already
        // been called.
        require(storage.end_block == 0, InitError::AlreadyInitialized);

        storage.end_block = height() + claim_time;
        storage.merkle_root = Option::Some(merkle_root);
        storage.asset = Option::Some(asset);
        storage.num_leaves = num_leaves;

        log(CreateAirdropEvent {
            asset,
            end_block: claim_time,
            merkle_root,
            num_leaves,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimState {
        storage.claims.get(identity).unwrap_or(ClaimState::Unclaimed)
    }

    #[storage(read)]
    fn end_block() -> u64 {
        storage.end_block
    }

    #[storage(read)]
    fn is_active() -> bool {
        storage.end_block > height()
    }

    #[storage(read)]
    fn merkle_root() -> Option<b256> {
        storage.merkle_root
    }
}
