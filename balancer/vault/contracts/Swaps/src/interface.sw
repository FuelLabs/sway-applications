library interface;

dep data_structures;

use data_structures::{
    SwapKind,
    SingleSwap,
    FundManagement,
    BatchSwapStep,
};

use std::{
    contract_id::ContractId,
    vec::Vec,
};

abi Swaps {
    #[storage(read)]
    fn swap(
        singleSwap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64
    ) -> u64;
    #[storage(read)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64
    ) -> Vec<u64>;
    #[storage(read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement
    ) -> Vec<u64>;
}