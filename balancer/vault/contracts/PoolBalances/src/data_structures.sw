library data_structures;

use std::{
    address::Address,
    contract_id::ContractId,
    vec::Vec,
};

pub enum UserBalanceOpKind { 
    DEPOSIT_INTERNAL: (),
    TRANSFER_EXTERNAL: (), 
    TRANSFER_INTERNAL: (),
    WITHDRAW_INTERNAL: (),
}

pub struct UserBalanceOp {
    amount: u64,
    asset: ContractId,
    kind: UserBalanceOpKind,
    recipient: Address,
    sender: Address,
}

pub enum PoolSpecialization { 
    GENERAL: (),
    MINIMAL_SWAP_INFO: (),
    TWO_TOKEN: ()
}

pub struct JoinPoolRequest {
    assets: Vec<ContractId>,
    maxAmountsIn: Vec<u64>,
    userData: b256,
    fromInternalBalance: bool,
}

pub struct ExitPoolRequest {
    assets: Vec<ContractId>,
    minAmountsOut: Vec<u64>,
    userData: b256,
    toInternalBalance: bool,
}

pub enum PoolBalanceChangeKind {
    JOIN: (),
    EXIT: (),
}

// This has the exact same layout as JoinPoolRequest and ExitPoolRequest, except the `maxAmountsIn` and
// `minAmountsOut` are called `limits`. Internally we use this struct for both since these two functions are quite
// similar, but expose the others to callers for clarity.
pub struct PoolBalanceChange {
    assets: Vec<ContractId>,
    limits: Vec<u64>,
    userData: b256,
    useInternalBalance: bool,
}