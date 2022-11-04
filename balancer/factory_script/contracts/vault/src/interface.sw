library interface;

dep data_structures;

use data_structures::{
    BatchSwapStep,
    ExitPoolRequest,
    FundManagement,
    JoinPoolRequest,
    PoolBalanceOp,
    PoolSpecialization,
    SingleSwap,
    SwapKind,
    UserBalanceOp,
};

use std::{address::Address, contract_id::ContractId, vec::Vec};

abi vault {
    #[storage(read, write)]fn batch_swap(kind: SwapKind, swaps: Vec<BatchSwapStep>, assets: Vec<ContractId>, funds: FundManagement, limits: Vec<u64>, deadline: u64) -> Vec<u64>;
    #[storage(read, write)]fn swap(singleSwap: SingleSwap, funds: FundManagement, limit: u64, deadline: u64) -> u64;
    #[storage(write, read)]fn query_batch_swap(kind: SwapKind, swaps: Vec<BatchSwapStep>, assets: Vec<ContractId>, funds: FundManagement) -> Vec<u64>;
    #[storage(read, write)]fn deregister_tokens(poolId: b256, tokens: Vec<ContractId>);
    #[storage(read, write)]fn register_tokens(poolId: b256, tokens: Vec<ContractId>, assetManagers: Vec<Address>);
    #[storage(read, write)]fn join_pool(poolId: b256, sender: Address, recipient: Address, request: JoinPoolRequest);
    #[storage(read, write)]fn exit_pool(poolId: b256, sender: Address, recipient: Address, request: ExitPoolRequest);
    #[storage(read, write)]fn flash_loan(recipient: ContractId, tokens: Vec<ContractId>, amounts: Vec<u64>, userData: Vec<b256>, );
    #[storage(read, write)]fn manage_pool_balance(ops: Vec<PoolBalanceOp>);
    #[storage(read, write)]fn manage_user_balance(ops: Vec<UserBalanceOp>);
    #[storage(read, write)]fn register_pool(specialization: PoolSpecialization) -> b256;
    #[storage(read, write)]fn set_authorizer(newAuthorizer: ContractId);
    #[storage(read, write)]fn set_relayer_approval(sender: Address, relayer: Address, approved: bool);
}
