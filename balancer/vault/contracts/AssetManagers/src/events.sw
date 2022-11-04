library events;

use std::{
    address::Address,
    contract_id::ContractId,
};

pub struct PoolBalanceManaged {
    pool_id: b256,
    sender: Address,
    token: ContractId,
    cash_delta: u64,
    managed_delta: u64,
}