library data_structures;

use std::contract_id::ContractId;

pub enum PoolSpecialization {
    GENERAL: (),
    MINIMAL_SWAP_INFO: (),
    TWO_TOKEN: (),
}

pub enum PoolBalanceOpKind { 
    WITHDRAW: (),
    DEPOSIT: (),
    UPDATE: (),
}

pub struct PoolBalanceOp {
    kind: PoolBalanceOpKind,
    poolId: b256,
    token: ContractId,
    amount: u64,
}