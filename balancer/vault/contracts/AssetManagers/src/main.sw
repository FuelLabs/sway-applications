contract;

dep events;
dep errors;
dep data_structures;
dep interface;
dep utils;

use events::PoolBalanceManaged;
use errors::Error;
use data_structures::{
    PoolSpecialization,
    PoolBalanceOp,
    PoolBalanceOpKind,
};
use interface::AssetManagers;
use utils::{
    is_token_registered,
    perform_pool_management_operation,
};


use std::{
    contract_id::ContractId,
    vec::Vec,
    storage::StorageMap,
    address::Address,
    identity::Identity,
    result::*,
    chain::auth::{AuthError, msg_sender},
    revert::{revert, require},
    context::call_frames::contract_id,
    token::transfer_to_output,
    option::Option,
    logging::log,
};


// use TwoTokenPoolsBalance::TwoTokenPoolsBalance;
// use GeneralPoolsBalance::GeneralPoolsBalance;
// use PoolRegistry::PoolRegistry;


// Stores the Asset Manager for each token of each Pool.
storage {
    pool_asset_managers: StorageMap<(b256, ContractId), Address> = StorageMap { },
    two_token_pools_balance_contract_id: ContractId = ContractId{ value: 0x0000000000000000000000000000000000000000000000000000000000000000},
    general_pools_balance_contract_id: ContractId = ContractId{ value: 0x0000000000000000000000000000000000000000000000000000000000000000},
    pool_registry_contract_id: ContractId = ContractId{ value: 0x0000000000000000000000000000000000000000000000000000000000000000},
}


// Performs the `kind` Asset Manager operation on a Pool.
//
// Withdrawals will transfer `amount` tokens to the caller, deposits will transfer `amount` tokens from the caller,
// and updates will set the managed balance to `amount`.
//
// Returns a tuple with the 'cash' and 'managed' balance deltas as a result of this call.
impl AssetManagers for Contract {
    #[storage(read)]
    fn manage_pool_balance(ops: Vec<PoolBalanceOp>) {
        // This variable could be declared inside the loop, but that causes the compiler to allocate memory on each
        // loop iteration, increasing gas costs.
        
        let mut count = 0;
        while count < ops.len() {
            // By indexing the array only once, we don't spend extra gas in the same bounds check.
            let op: PoolBalanceOp = ops.get(count).unwrap();

            let pool_id: b256 = op.poolId;
            // let x = abi(PoolRegistry, pool_registry_contract_id);
            // x.ensure_registered_pool(poolId);

            let sender: Result<Identity, AuthError> = msg_sender();
            let sender: Address = match sender.unwrap() {
                Identity::Address(addr) => {
                    addr
                },
                _ => {
                    revert(0);
                },
            };
            let token: ContractId = op.token;
            require(is_token_registered(pool_id, token), Error::TOKEN_NOT_REGISTERED);
            require(storage.pool_asset_managers.get((pool_id, token)) == sender, Error::SENDER_NOT_ASSET_MANAGER);

            let kind: PoolBalanceOpKind = op.kind;
            let amount = op.amount;
            let(cash_delta, managed_delta) = perform_pool_management_operation(kind, pool_id, token, amount);
            count = count + 1;

            log(
                PoolBalanceManaged {
                    pool_id: pool_id,
                    sender: sender,
                    token: token,
                    cash_delta: cash_delta,
                    managed_delta: managed_delta,
                }
            );
        }
    }
}
