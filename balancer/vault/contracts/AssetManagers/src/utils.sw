library utils;

dep data_structures;

use data_structures::{
    PoolBalanceOpKind,
    PoolSpecialization,
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

pub fn perform_pool_management_operation(
    kind: PoolBalanceOpKind,
    poolId: b256,
    token: ContractId,
    amount: u64
) -> (u64, u64) {
    // let x = abi(PoolRegistry, pool_registry_contract_id);
    // let specialization: PoolSpecialization = x.get_pool_specialization(poolId);
    // if let PoolBalanceOpKind::WITHDRAW = kind {
    //     return withdraw_pool_balance(poolId, specialization, token, amount);
    // } else if let PoolBalanceOpKind::DEPOSIT = kind {
    //     return deposit_pool_balance(poolId, specialization, token, amount);
    // } else {
    //     // PoolBalanceOpKind::UPDATE
    //     return update_managed_balance(poolId, specialization, token, amount);
    // }
    (0,0)
}


// Moves `amount` tokens from a Pool's 'cash' to 'managed' balance, and transfers them to the caller.
// Returns the 'cash' and 'managed' balance deltas as a result of this call, which will be complementary.
fn withdraw_pool_balance(
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64
) -> (u64, u64) {
    // let x = abi(TwoTokenPoolsBalance, two_token_pools_balance_contract_id);
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     x._two_token_pool_cash_to_managed(poolId, token, amount);
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     x._minimal_swap_info_pool_cash_to_managed(poolId, token, amount);
    // } else {
    //     // PoolSpecialization::GENERAL
    //     let x = abi(GeneralPoolsBalance, general_pools_balance_contract_id);
    //     x._general_pool_cash_to_managed(poolId, token, amount);
    // }


    if (amount > 0) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        transfer_to_output(amount, token, sender);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(-amount);
    // managedDelta = int256(amount);
    return (amount, amount);
}

// Moves `amount` tokens from a Pool's 'managed' to 'cash' balance, and transfers them from the caller.
//
// Returns the 'cash' and 'managed' balance deltas as a result of this call, which will be complementary.

fn deposit_pool_balance(
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64
) -> (u64, u64) {
    // let x = abi(TwoTokenPoolsBalance, two_token_pools_balance_contract_id);
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     x.two_token_pool_managed_to_cash(poolId, token, amount);
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     x.minimal_swap_info_pool_managed_to_cash(poolId, token, amount);
    // } else {
    //     // PoolSpecialization::GENERAL
    //     let x = abi(GeneralPoolsBalance, general_pools_balance_contract_id);
    //     x.general_pool_managed_to_cash(poolId, token, amount);
    // }

    if (amount > 0) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        transfer_to_output(amount, contract_id(), sender);
        // token.safeTransferFrom(msg.sender, address(this), amount);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(amount);
    // managedDelta = int256(-amount);
    return (amount, amount);
}

// Sets a Pool's 'managed' balance to `amount`.
//
// Returns the 'cash' and 'managed' balance deltas as a result of this call (the 'cash' delta will always be zero).
fn update_managed_balance(
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64
) -> (u64, u64) {
    let mut managedDelta = 0;
    // let x = abi(TwoTokenPoolsBalance, two_token_pools_balance_contract_id);
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     let managedDelta = x.set_two_token_pool_managed_balance(poolId, token, amount);
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     let managedDelta = x.set_minimal_swap_info_pool_managed_balance(poolId, token, amount);
    // } else {
    //     // PoolSpecialization::GENERAL
    //     let x = abi(GeneralPoolsBalance, general_pools_balance_contract_id);
    //     let managedDelta = x.set_general_pool_managed_balance(poolId, token, amount);
    // }

    // cashDelta = 0;
    return (0, managedDelta)
}

// Returns true if `token` is registered for `poolId`.
pub fn is_token_registered(poolId: b256, token: ContractId) -> bool {
    // let x = abi(PoolRegistry, pool_registry_contract_id);
    // let specialization: PoolSpecialization = get_pool_specialization(poolId);
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     return x.is_two_token_pool_token_registered(poolId, token);
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     return x.is_minimal_swap_info_pool_token_registered(poolId, token);
    // } else {
    //     // PoolSpecialization::GENERAL
    //     let x = abi(GeneralPoolsBalance, general_pools_balance_contract_id);
    //     return x.is_general_pool_token_registered(poolId, token);
    // }
    true
}


