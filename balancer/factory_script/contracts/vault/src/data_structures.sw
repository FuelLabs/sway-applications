library data_structures;

pub const MASK = 1;

use std::{address::Address, contract_id::ContractId, vec::Vec};

pub enum SwapKind {
    GIVEN_IN: (),
    GIVEN_OUT: (),
}

pub struct SingleSwap {
    poolId: b256,
    kind: SwapKind,
    assetIn: ContractId,
    assetOut: ContractId,
    amount: u64,
    userData: b256,
}

pub struct FundManagement {
    sender: Address,
    fromInternalBalance: bool,
    recipient: Address,
    toInternalBalance: bool,
}

pub struct SwapRequest {
    kind: SwapKind,
    tokenIn: ContractId,
    tokenOut: ContractId,
    amount: u64,
    // Misc data
    poolId: b256,
    lastChangeBlock: u64,
    from: Address,
    to: Address,
    userData: b256,
}

pub struct BatchSwapStep {
    poolId: b256,
    assetInIndex: u64,
    assetOutIndex: u64,
    amount: u64,
    userData: b256,
}

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

pub struct abi_encode {
    token_a: ContractId,
    token_b: ContractId,
}

pub struct TwoTokenPoolBalances {
    shared_cash: b256,
    shared_managed: b256,
}

pub struct TwoTokenPoolTokens {
    token_a: ContractId,
    token_b: ContractId,
    // workaround of nested storageMap
    // balances: StorageMap<b256, TwoTokenPoolBalances>,
    balances: b256,
}

pub struct IERC20ToBytes32MapEntry {
    key: ContractId,
    value: b256,
}

pub struct IERC20ToBytes32Map {
    // Number of entries in the map
    length: u64,
    // Storage of map keys and values
    entries: u64,
    // Position of the entry defined by a key in the `entries` array, plus 1
    // because index 0 means a key is not in the map.
    indexes: ContractId,
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

// This has the exact same layout as JoinPoolRequest and ExitPoolRequest, except the `maxAmountsIn` and
// `minAmountsOut` are called `limits`. Internally we use this struct for both since these two functions are quite
// similar, but expose the others to callers for clarity.
pub struct PoolBalanceChange {
    assets: Vec<ContractId>,
    limits: Vec<u64>,
    userData: b256,
    useInternalBalance: bool,
}

pub enum PoolBalanceChangeKind {
    JOIN: (),
    EXIT: (),
}
