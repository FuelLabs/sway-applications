library events;

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

pub struct TransferEvent {
    asset: ContractId,
    nonce: u64,
    to: Identity,
    value: u64,
}
