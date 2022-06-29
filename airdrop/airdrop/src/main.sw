contract;

dep abi;
dep data_structures;
dep events;
dep errors;
dep utils;

use abi::Airdrop;
use data_structures::{Claim, State};
use events::ClaimEvent;
use errors::AccessError;
use utils::{create_hash};
use std::{
    logging::log,
    storage::StorageMap,
};

storage {
    has_claimed: StorageMap<Identity, bool>,
    token_contract: ContractId,
    state: State,
}

impl Airdrop for Contract {
    /// This fucntion will let users claim their airdrop
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
}
