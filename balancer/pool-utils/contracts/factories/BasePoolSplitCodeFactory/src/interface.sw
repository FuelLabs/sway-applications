library interface;

use std::address::Address;

abi BasePoolSplitCodeFactory {
    #[storage(read)]fn is_pool_from_factory(address: Address) -> bool;
    #[storage(read, write)] fn disable() -> ();
    #[storage(read, write)] fn _create() -> Address;
}