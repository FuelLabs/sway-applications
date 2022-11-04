library events;

dep data_structures;

use data_structures::PoolSpecialization;

use std::{address::Address, contract_id::ContractId, vec::Vec};

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

pub struct PoolBalanceManaged {
    pool_id: b256,
    sender: Address,
    token: ContractId,
    cash_delta: u64,
    managed_delta: u64,
}

pub struct TokensRegistered {
    pool_id: b256,
    tokens: Vec<ContractId>,
    asset_managers: Vec<Address>,
}

pub struct TokensDeregistered {
    poolId: b256,
    tokens: Vec<ContractId>,
}

pub struct PoolBalanceChanged {
    pool_id: b256,
    sender: Address,
    tokens: Vec<ContractId>,
    // We can unsafely cast to int256 because balances are actually stored as uint112
    balances: Vec<u64>,
    paid_protocol_swap_fee_amounts: Vec<u64>,
}

pub struct FlashLoan {
    recipient: ContractId,
    token: ContractId,
    amount: u64,
    receivedFeeAmount: u64,
}

pub struct AuthorizerChanged {
    newAuthorizer: ContractId,
}

pub struct RelayerApprovalChanged {
    sender: Address,
    relayer: Address,
    approved: bool,
}

pub struct PoolRegistered {
    poolId: b256,
    address: Address,
    specialization: PoolSpecialization,
}
