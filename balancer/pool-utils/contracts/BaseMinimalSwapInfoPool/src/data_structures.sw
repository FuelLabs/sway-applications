library data_structures;

use std::{
    address::Address,
    contract_id::ContractId
};

pub enum SwapKind { 
    GIVEN_IN: (),
    GIVEN_OUT: (),
}

pub struct SwapRequest {
    kind: SwapKind,
    tokenIn: ContractId,
    tokenOut: ContractId,
    amount: u64,
    poolId: b256,
    lastChangeBlock: u64,
    from: Address,
    to: Address,
    userData: b256,
}