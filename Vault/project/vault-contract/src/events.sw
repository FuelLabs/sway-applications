library events;

pub struct DepositEvent {
    assets: u64,
    owner: Identity,
    sender: Identity,
    shares: u64,
}

pub struct MintEvent {}

pub struct RedeemEvent {}

pub struct WithdrawEvent {
    assets: u64,
    owner: Identity,
    receiver: Identity,
    sender: Identity,
    shares: u64,
}
