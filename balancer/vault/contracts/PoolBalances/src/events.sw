library events;

use std::{
    address::Address,
    contract_id::ContractId,
};

pub struct ExternalBalanceTransfer {
    amount: u64,
    recipient: Address,
    sender: Address,
    token: ContractId,
}

pub struct InternalBalanceChanged {
    account: Address,
    delta: u64,
    token: ContractId,
}