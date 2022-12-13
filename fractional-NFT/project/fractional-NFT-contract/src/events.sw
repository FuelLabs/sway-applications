library events;

pub struct Deposited {
    nft: ContractId,
    owner: Identity,
    supply: u64,
    token_id: u64,
}

pub struct OwnerChanged {
    new_owner: Option<Identity>,
    previous_owner: Identity,
}

pub struct Withdraw {
    nft: ContractId,
    owner: Identity,
    token_id: u64,
}
