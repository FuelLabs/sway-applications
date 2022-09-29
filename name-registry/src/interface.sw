library interface;

use std::identity::Identity;

abi NameRegistry {
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64);
    #[storage(read, write)]
    fn register(name: str[8], duration: u64);
    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity);
    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity);
}