library events;


pub struct ExecutedEvent {
    data: b256, // TODO: change to vector when implemented
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
