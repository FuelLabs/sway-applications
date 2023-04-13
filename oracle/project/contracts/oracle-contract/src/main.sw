contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use std::auth::msg_sender;

use data_structures::State;
use errors::AccessError;
use events::PriceUpdateEvent;
use interface::Oracle;

storage {
    // Current price of tracked asset
    price: Option<u64> = Option::None,
}

// TODO treat owner as an identity once https://github.com/FuelLabs/sway/issues/2647 is fixed
impl Oracle for Contract {
    fn owner() -> Identity {
        Identity::Address(Address::from(OWNER))
    }

    #[storage(read)]
    fn price() -> Option<u64> {
        storage.price
    }

    #[storage(write)]
    fn set_price(price: u64) {
        require(msg_sender().unwrap() == Identity::Address(Address::from(OWNER)), AccessError::NotOwner);

        storage.price = Option::Some(price);

        log(PriceUpdateEvent { price });
    }
}
