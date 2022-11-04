contract;

dep interface;

use std::{address::Address, revert::require, storage::StorageMap};
use BalancerErrors::DISABLED;

use interface::BasePoolSplitCodeFactory;

storage {
    _is_pool_from_factory: StorageMap<Address,
    bool> = StorageMap {
    },
    _disabled: bool = false,
}

#[storage(read)]fn is_disabled() -> bool {
    return storage._disabled;
}

#[storage(read)]fn _ensure_enabled() -> () {
    require(!is_disabled(), DISABLED);
}

impl BasePoolSplitCodeFactory for Contract {
    #[storage(read)]fn is_pool_from_factory(address: Address) -> bool {
        storage._is_pool_from_factory.get(address)
    }

    #[storage(read, write)] fn disable() -> () {
        _ensure_enabled();
        storage._disabled = true;
    }

    #[storage(read, write)] fn _create() -> Address {
        _ensure_enabled();
        // we need to call this to create a pool address, below is the dummy code.
        // address pool = super._create(constructorArgs);
        const _SOME_ADDRESS: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
        let address = ~Address::from(_SOME_ADDRESS);
        storage._is_pool_from_factory.insert(address, true);

        return address;
    }
}
