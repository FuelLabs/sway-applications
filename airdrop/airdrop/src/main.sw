contract;

dep abi;
dep data_structures;
dep events;
dep errors;
dep utils;

use abi::Airdrop;
use data_structures::{Claim, State};
use events::ClaimEvent;
use errors::{AccessError, StateError};
use utils::{create_hash};
use std::{
    logging::log,
    storage::StorageMap,
};

storage {
    has_claimed: StorageMap<Identity, bool>,
    token_contract: ContractId,
    state: State,
    merkleRoot: b256,
}

impl Airdrop for Contract {
    /// This function will let users claim their airdrop
    ///
    /// # Reverts
    /// - The `to` `Identity` has already claimed
    #[storage(read,write)]fn claim(to: Identity, amount: u64, bytes: b256) {
        require(!has_claimed.get(to), AccessError::UserAlreadyClaimed);

        let hash = create_hash(to, amount);
        
        // Verify valid leaf

        storage.has_claimed.insert(to, true);
        mint_to(token_contract, to, amount);

        // Event
        log(ClaimEvent{
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
