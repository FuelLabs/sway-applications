contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use std::{
    address::Address,
    chain::auth::msg_sender,
    identity::Identity,
    logging::log,
    result::Result,
    revert::require,
};

use data_structures::State;
use errors::AccessError;
use events::PriceUpdateEvent;
use interface::Oracle;

storage {
    // Current price of tracked asset
    // TODO use option when https://github.com/FuelLabs/fuels-rs/issues/415 is fixed
    price: u64 = 0,
}

// TODO treat owner as an identity once https://github.com/FuelLabs/sway/issues/2647 is fixed
impl Oracle for Contract {
    fn owner() -> Identity {
        Identity::Address(~Address::from(owner))
    }

    #[storage(read)] fn price() -> u64 {
        storage.price
    }

    #[storage(write)] fn set_price(price: u64) {
        require(msg_sender().unwrap() == Identity::Address(~Address::from(owner)), AccessError::NotOwner);

        storage.price = price;

        log(PriceUpdateEvent {
            price
        });
    }
}
