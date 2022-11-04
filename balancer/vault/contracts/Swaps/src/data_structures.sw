library data_structures;

use std::{
    address::Address,
    contract_id::ContractId,
};

pub const TOKEN_NOT_REGISTERED = 512;

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

