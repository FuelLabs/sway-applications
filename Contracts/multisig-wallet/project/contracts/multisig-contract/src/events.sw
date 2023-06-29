library;

use ::data_structures::user::User;

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

pub struct SetWeightEvent {
    user: User,
}

pub struct TransferEvent {
    asset: ContractId,
    nonce: u64,
    to: Identity,
    value: u64,
}
