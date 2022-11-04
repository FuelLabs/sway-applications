contract;

dep events;
dep errors;
dep data_structures;
dep interface;
dep utils;

use events::{
    PoolBalanceManaged,
    TokensRegistered,
    TokensDeregistered,
};
use errors::Error;
use data_structures::{
    PoolSpecialization,
    PoolBalanceOp,
    PoolBalanceOpKind,
};
use interface::PoolTokens;
use utils::{
    is_token_registered,
    perform_pool_management_operation,
};

use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    contract_id::ContractId,
    vec::Vec,
    option::Option,
    reentrancy::is_reentrant,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output},
    logging::log,
    constants::ZERO_B256,
    result::*,
    identity::Identity,
    revert::{revert, require},
};

use BalancerErrors::{
    INVALID_TOKEN, 
    TOKENS_LENGTH_MUST_BE_2
};
use BalanceAllocation::{
    cash,
    last_change_block,
    managed, 
    totals_and_last_change_block
};
use InputHelpers::ensure_input_length_match;

// use PoolRegistry::{PoolSpecialization, _get_pool_specialization, with_registered_pool};
// use TemporarilyPausable::TemporarilyPausable;
// use TwoTokenPoolBalance::TwoTokenPoolBalance;
// use MinimalSwapInfoPoolBalance::MinimalSwapInfoPoolBalance;
// use GeneralPoolsBalance::GeneralPoolsBalance;

storage {
    pool_asset_managers: StorageMap<(b256, ContractId), Address> = StorageMap { },
    asset_managers_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    pool_registry_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    temporarily_pausable_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    two_token_pools_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    minimal_swap_info_pool_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    general_pools_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,

}


impl PoolTokens for Contract {
    // this function originally belongs to AssetManagers contract
    #[storage(read)]
    fn manage_pool_balance(ops: Vec<PoolBalanceOp>) {
        // This variable could be declared inside the loop, but that causes the compiler to allocate memory on each
        // loop iteration, increasing gas costs.
        
        let mut count = 0;
        while count < ops.len() {
            // By indexing the array only once, we don't spend extra gas in the same bounds check.
            let op: PoolBalanceOp = ops.get(count).unwrap();

            let pool_id: b256 = op.poolId;
            // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
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

    #[storage(read, write)]
    fn register_tokens(poolId: b256, tokens: Vec<ContractId>, assetManagers: Vec<Address>) {
        is_reentrant();
        // let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
        // x.ensure_not_paused();
        ensure_input_length_match(tokens.len(), assetManagers.len());

        // Validates token addresses and assigns Asset Managers
        let mut count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            require(token != ~ContractId::from(ZERO_B256), INVALID_TOKEN);
            storage.pool_asset_managers.insert((poolId, token), assetManagers.get(count).unwrap());
        }
        // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        // let specialization: PoolSpecialization = x.get_pool_specialization(poolId);
        // if let PoolSpecialization::TWO_TOKEN = specialization{
        //     require(tokens.len() == 2, Errors::TOKENS_LENGTH_MUST_BE_2);
        //     let x = abi(TwoTokenPoolBalance, storage.two_token_pools_balance_contract_id);
        //     x.register_two_token_pool_tokens(poolId, tokens.get(0).unwrap(), tokens.get(1).unwrap());
        // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
        //     let x = abi(MinimalSwapInfoPoolBalance, storage.minimal_swap_info_pool_balance_contract_id);
        //     x.register_minimal_swap_info_pool_tokens(poolId, tokens);
        // } else {
        //     let x = abi(GeneralPoolTokens, storage.general_pools_balance_contract_id);
        //     x.register_general_pool_tokens(poolId, tokens);
        // }
        log(
            TokensRegistered{
                pool_id: poolId, 
                tokens: tokens, 
                asset_managers: assetManagers,
            }
        );
        
    }

    #[storage(read, write)]
    fn deregister_tokens(poolId: b256, tokens: Vec<ContractId>) {
        is_reentrant();
        let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
        x.ensure_not_paused();
        let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        let specialization: PoolSpecialization = x.get_pool_specialization(poolId);
        if let PoolSpecialization::TWO_TOKEN = specialization {
            _require(tokens.len() == 2, TOKENS_LENGTH_MUST_BE_2);
            let x = abi(TwoTokenPoolBalance, storage.two_token_pools_balance_contract_id);
            x.deregister_two_token_pool_tokens(poolId, tokens.get(0).unwarp(), tokens.get(1).unwrap());
        } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
            let x = abi(MinimalSwapInfoPoolBalance, storage.minimal_swap_info_pool_balance_contract_id);
            x.deregister_minimal_swap_info_pool_tokens(poolId, tokens);
        } else {
            // PoolSpecialization::GENERAL
            let x = abi(GeneralPoolTokens, storage.general_pools_balance_contract_id);
            x.deregister_general_pool_tokens(poolId, tokens);
        }

        // The deregister calls above ensure the total token balance is zero. Therefore it is now safe to remove any
        // associated Asset Managers, since they hold no Pool balance.
        // Todo need to be implemented when we can remove things from storage
        // let mut count = 0;
        // while count < tokens.len() {
        //     storage.pool_asset_managers.insert((poolId, tokens.get(count).unwarp()), ~Address::from(ZERO_B256));
        //     count = count + 1;
        // }
        log(
            TokensDeregistered{
            poolId: poolId,
            tokens: tokens,
            }
        );
    }

    #[storage(read)]
    fn get_pool_tokens(poolId: b256) -> (Vec<ContractId>, Vec<u64>, u64) {
        // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        // x.with_registered_pool(poolId);
        let rawBalances: Vec<b256> = ~Vec::new();
        let(tokens, rawBalances) = private_get_pool_tokens(poolId);
        let(balances, lastChangeBlock) = totals_and_last_change_block(rawBalances);
        return(tokens, balances, lastChangeBlock)
    }

    #[storage(read)]
    fn get_pool_token_info(poolId: b256, token: ContractId) -> (u64, u64, u64, Address) {
        // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        // x.with_registered_pool(poolId);
        let mut balance: b256 = ZERO_B256;
        let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        let specialization: PoolSpecialization = x.get_pool_specialization(poolId);

        if let PoolSpecialization::TWO_TOKEN = specialization {
            let x = abi(TwoTokenPoolBalance, storage.two_token_pools_balance_contract_id);
            balance = x.get_two_token_pool_balance(poolId, token);
        } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
            let x = abi(MinimalSwapInfoPoolBalance, storage.minimal_swap_info_pool_balance_contract_id);
            balance = x.get_minimal_swap_info_pool_balance(poolId, token);
        } else {
            // PoolSpecialization::GENERAL
            let x = abi(GeneralPoolsBalance, storage.general_pools_balance_contract_id);
            balance = x.get_general_pool_balance(poolId, token);
        }

        return(
            cash(balance), 
            managed(balance), 
            last_change_block(balance),
            storage.pool_asset_managers.get((poolId, token)),
        )
    }

}

#[storage(read)]
// Returns all of `poolId`'s registered tokens, along with their raw balances.
fn private_get_pool_tokens(poolId: b256) -> (Vec<ContractId>, Vec<b256>) {
    let x = abi(PoolRegistry, storage.pool_registry_contract_id);
    let specialization: PoolSpecialization = x.get_pool_specialization(poolId);
    if let PoolSpecialization::TWO_TOKEN = specialization {
        let x = abi(TwoTokenPoolBalance, storage.two_token_pools_balance_contract_id);
        return x.get_two_token_pool_tokens(poolId);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
        let x = abi(MinimalSwapInfoPoolBalance, storage.minimal_swap_info_pool_balance_contract_id);
        return x.get_minimal_swap_info_pool_tokens(poolId);
    } else {
        // PoolSpecialization::GENERAL
        let x = abi(GeneralPoolsBalance, storage.general_pools_balance_contract_id);
        return x.get_general_pool_tokens(poolId);
    }
}