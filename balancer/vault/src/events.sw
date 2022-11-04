library events;

dep data_structures;

use data_structures::PoolSpecialization;

use std::{address::Address, contract_id::ContractId, vec::Vec};

pub struct EventExternalBalanceTransfer {
    amount: u64,
    recipient: Address,
    sender: Address,
    token: ContractId,
}

pub struct EventInternalBalanceChanged {
    account: Address,
    delta: u64,
    token: ContractId,
}

pub struct EventPoolBalanceManaged {
    pool_id: b256,
    sender: Address,
    token: ContractId,
    cash_delta: u64,
    managed_delta: u64,
}

pub struct EventTokensRegistered {
    pool_id: b256,
    tokens: Vec<ContractId>,
    asset_managers: Vec<Address>,
}

pub struct EventTokensDeregistered {
    pool_id: b256,
    tokens: Vec<ContractId>,
}

pub struct EventPoolBalanceChanged {
    pool_id: b256,
    sender: Address,
    tokens: Vec<ContractId>,
    // We can unsafely cast to int256 because balances are actually stored as uint112
    balances: Vec<u64>,
    paid_protocol_swap_fee_amounts: Vec<u64>,
}

pub struct EventFlashLoan {
    recipient: ContractId,
    token: ContractId,
    amount: u64,
    received_fee_amount: u64,
}

pub struct EventAuthorizerChanged {
    new_authorizer: ContractId,
}

pub struct EventRelayerApprovalChanged {
    sender: Address,
    relayer: Address,
    approved: bool,
}

pub struct EventPoolRegistered {
    pool_id: b256,
    specialization: PoolSpecialization,
}

// Emitted for each individual swap performed by `swap` or `batchSwap`.
pub struct EventSwap {   
    pool_id: b256,
    token_in: ContractId,
    token_out: ContractId,
    amount_in: u64,
    amount_out: u64,
}
