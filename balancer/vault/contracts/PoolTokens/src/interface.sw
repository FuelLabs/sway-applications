library interface;

dep data_structures;

use data_structures::PoolBalanceOp;

use std::{
    vec::Vec,
    contract_id::ContractId,
    address::Address,
};

abi PoolTokens {
    #[storage(read)]fn manage_pool_balance(ops: Vec<PoolBalanceOp>);
    #[storage(read, write)]fn register_tokens(poolId: b256, tokens: Vec<ContractId>, assetManagers: Vec<Address>);
    #[storage(read, write)]fn deregister_tokens(poolId: b256, tokens: Vec<ContractId>);
    #[storage(read)]fn get_pool_tokens(poolId: b256) -> (Vec<ContractId>, Vec<u64>, u64);
    #[storage(read)]fn get_pool_token_info(poolId: b256, token: ContractId) -> (u64, u64, u64, Address);
}
