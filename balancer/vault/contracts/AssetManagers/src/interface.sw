library interface;

dep data_structures;

use data_structures::PoolBalanceOp;

use std::vec::Vec;

abi AssetManagers {
    #[storage(read)]fn manage_pool_balance(ops: Vec<PoolBalanceOp>);
}