contract;

mod data_structures;
mod errors;
mod events;
mod interface;

use ::data_structures::State;
use ::errors::AccessError;
use ::events::PriceUpdateEvent;
use ::interface::Oracle;
use std::auth::msg_sender;

configurable {
    OWNER: Identity = Identity::Address(Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db)),
}

storage {
    // Current price of tracked asset
    price: Option<u64> = Option::None,
}

impl Oracle for Contract {
    fn owner() -> Identity {
        OWNER
    }

    #[storage(read)]
    fn price() -> Option<u64> {
        match storage.price.try_read() {
            Option::Some(price) => price,
            Option::None => Option::None,
        }
    }

    #[storage(write)]
    fn set_price(price: u64) {
        require(msg_sender().unwrap() == OWNER, AccessError::NotOwner);

        storage.price.write(Option::Some(price));

        log(PriceUpdateEvent { price });
    }
}
