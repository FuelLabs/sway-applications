contract;

dep interface;
dep data_structures;
dep errors;
dep events;

use std::{
    chain::auth::msg_sender,
    constants::BASE_ASSET_ID,
    identity::Identity,
    logging::log,
    result::Result,
    revert::require,
};

use interface::Oracle;
use data_structures::State;
use errors::{AccessError, InitializationError};
use events::PriceUpdateEvent;

storage {
    /// The Identity that can control the oracle (node)
    owner: Identity = Identity::ContractId(BASE_ASSET_ID),
    /// Current price of tracked asset
    price: u64 = 0,
    /// The initialization state of the contract.
    state: State = State::NotInitialized,
}

impl Oracle for Contract {
    #[storage(read, write)] fn constructor(owner: Identity) {
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.owner = owner;
        storage.state = State::Initialized;
    }

    #[storage(read, write)] fn set_price(new_price: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        let sender = msg_sender().unwrap();
        require(sender == storage.owner, AccessError::NotOwner);

        storage.price = new_price;

        log(PriceUpdateEvent {
            price: new_price
        });
    }

    #[storage(read)] fn price() -> u64 {
        storage.price
    }
}
