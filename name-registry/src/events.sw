library events;

use std::identity::Identity;

pub struct IdentityChanged {
    name: str[8],
    new_identity: Identity,
    old_identity: Identity,
}

pub struct NameRegistered {
    expiry: u64,
    name: str[8],
    owner: Identity,
}

pub struct OwnerChanged {
    name: str[8],
    new_owner: Identity,
    old_owner: Identity,
}

pub struct RegistrationExtended {
    duration: u64,
    name: str[8],
    new_expiry: u64,
}
