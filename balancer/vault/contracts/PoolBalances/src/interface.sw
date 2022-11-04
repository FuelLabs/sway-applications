library interface;

dep data_structures;

use data_structures::{
    UserBalanceOp,
    JoinPoolRequest,
    ExitPoolRequest,
};

use std::{
    vec::Vec,
    contract_id::ContractId,
    address::Address,
};

abi PoolBalances {
    fn join_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest
    );
    fn exit_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: ExitPoolRequest
    );
}
