contract;

dep interface;
dep data_structures;
dep errors;
dep events;

use std::{
    address::Address,
    chain::auth::msg_sender,
    constants::BASE_ASSET_ID,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    revert::require,
};

use interface::Oracle;
use data_structures::State;
use errors::AccessError;
use events::PriceUpdateEvent;

storage {
    // Current price of tracked asset
    // TODO use option when https://github.com/FuelLabs/fuels-rs/issues/415 is fixed
    price: u64 = 0,
}

impl Oracle for Contract {
    fn owner() -> Identity {
        Identity::Address(~Address::from(owner))
    }

    #[storage(read)] fn price() -> u64 {
        storage.price
    }

    #[storage(read, write)] fn set_price(price: u64) {
        require(msg_sender().unwrap() == Identity::Address(~Address::from(owner)), AccessError::NotOwner);

        storage.price = price;

        log(PriceUpdateEvent {
            price
        });
    }
}
