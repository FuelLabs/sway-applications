library events;

use std::{
    address::Address,
    contract_id::ContractId,
    vec::Vec,
};

pub struct PoolBalanceManaged {
    pool_id: b256,
    sender: Address,
    token: ContractId,
    cash_delta: u64,
    managed_delta: u64,
}

pub struct TokensRegistered{
    pool_id: b256,
    tokens:Vec<ContractId>, 
    asset_managers: Vec<Address>
}

pub struct TokensDeregistered{
    poolId: b256,
    tokens:Vec<ContractId>
}