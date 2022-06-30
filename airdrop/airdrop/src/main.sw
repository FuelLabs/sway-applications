contract;

dep abi;
dep data_structures;
dep events;
dep errors;
dep utils;

use abi::Airdrop;
use data_structures::{Claim, State};
use events::ClaimEvent;
use errors::{AccessError, StateError, VerificationError};
use utils::{create_hash, mint, verify};
use std::{
    assert::require,
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    storage::StorageMap,
    vec::Vec,
};

storage {
    has_claimed: StorageMap<Identity,
    bool>, token_contract: ContractId,
    state: State,
    merkleRoot: b256,
}

impl Airdrop for Contract {
    /// This function will let users claim their airdrop
    ///
    /// # Reverts
    /// - The `to` `Identity` has already claimed
    #[storage(read, write)]fn claim(to: Identity, amount: u64, bytes: Vec<b256>) {
        require(!storage.has_claimed.get(to), AccessError::UserAlreadyClaimed);

        let hash = create_hash(to, amount);

        // Verify valid leaf
        require(verify(storage.merkleRoot, hash, bytes), VerificationError::MerkleProofFailed);

        storage.has_claimed.insert(to, true);
        mint(storage.token_contract, to, amount);

        // Event
        log(ClaimEvent {
            to, amount
        });
    }

    /// Initalizes the contract
    ///
    /// # Reverts
    ///
    /// - The contract has already been intialized
    #[storage(read, write)]fn constructor(token_contract: ContractId, merkleRoot: b256) {
        require(storage.state == State::NotInitalized, StateError::AlreadyInitalized);
        storage.state = State::Initalized;
        storage.token_contract = token_contract;
        storage.merkleRoot = merkleRoot;
    }
}
