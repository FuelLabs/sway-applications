library events;

dep data_structures/user;

use user::User;

pub struct AddedOwnersEvent {
    users: Vec<User>,
}

pub struct CancelEvent {
    cancelled_nonce: u64,
    user: b256,
}

pub struct ExecutedEvent {
    data: b256, // TODO: change to Bytes when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723
    nonce: u64,
    to: Identity,
    value: u64,
}

pub struct SetThresholdEvent {
    previous_threshold: u64,
    threshold: u64,
}

pub struct SetWeightsEvent {
    users: Vec<User>,
}

pub struct TransferEvent {
    asset: ContractId,
    nonce: u64,
    to: Identity,
    value: u64,
}
