contract;

use std::storage::StorageMap;
use std::block::timestamp;
use std::assert::assert;
use std::context::msg_amount;
use std::context::call_frames::msg_asset_id;
use std::constants::BASE_ASSET_ID;
use std::constants::ZERO_B256;
use std::chain::auth::msg_sender;


pub struct Record {
    owner: Identity,
    address: Identity,
    expiry: u64,
}

abi MyContract {
    #[storage(read, write)]
    fn register(name: str[8], duration: u64);
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64);
}

storage {
    names: StorageMap<str[8], Option<Record>> = StorageMap {},
}

const PRICE_PER_HUNDRED: u64 = 1;

impl MyContract for Contract {
    #[storage(read, write)]
    fn register(name: str[8], duration: u64) {
        if storage.names.get(name).is_some() { 
            let record = storage.names.get(name).unwrap();
            assert(timestamp() > record.expiry);
        }

        assert(duration/100 * PRICE_PER_HUNDRED <= msg_amount());
        assert(msg_asset_id() == BASE_ASSET_ID);

        storage.names.insert(name, Option::Some(Record {
            owner: msg_sender().unwrap(),
            address: msg_sender().unwrap(),
            expiry: timestamp() + duration,
        }));
    }

    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        assert(storage.names.get(name).is_some());
        assert(duration/100 * PRICE_PER_HUNDRED <= msg_amount());

        let record = storage.names.get(name).unwrap();

        storage.names.insert(name, Option::Some(Record {
            owner: record.owner,
            address: record.address,
            expiry: record.expiry + duration,
        }))
    }

    
}
