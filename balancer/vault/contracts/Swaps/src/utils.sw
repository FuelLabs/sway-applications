library utils;
dep errors;
dep data_structures;

use errors::Error;
use data_structures::{
    SwapKind,
    SwapRequest,
};
use math::{
    add,
    max,
};
use BalanceAllocation::{
    last_change_block,
    increase_cash,
    decrease_cash,
    total,
};

use std::{
    vec::Vec,
    contract_id::ContractId,
    option::Option,
    address::Address,
    token::{force_transfer_to_contract,transfer_to_output},
    chain::auth::{AuthError, msg_sender},
    revert::{revert, require},
    math::*,
    identity::Identity,
    result::*,
    context::{call_frames::contract_id, msg_amount},
};

// For `swap_with_pool` to handle both 'given in' and 'given out' swaps, it internally tracks the 'given' amount
// (supplied by the caller), and the 'calculated' amount (returned by the Pool in response to the swap request).

// Given the two swap tokens and the swap kind, returns which one is the 'given' token (the token whose
// amount is supplied by the caller).
pub fn token_given(
    kind: SwapKind,
    tokenIn: ContractId,
    tokenOut: ContractId,
) -> ContractId 
{
    if let SwapKind::GIVEN_IN = kind {
        return tokenIn;
    }
    else {
        return tokenOut;    
    }
}

// Given the two swap tokens and the swap kind, returns which one is the 'calculated' token (the token whose
// amount is calculated by the Pool).
pub fn token_calculated(
    kind: SwapKind,
    tokenIn: ContractId,
    tokenOut: ContractId
) -> ContractId {
    if let SwapKind::GIVEN_IN = kind {
        return tokenOut;
    }
    else {
        return tokenIn;
    }
}

/// Returns excess ETH back to the contract caller, assuming `amountUsed` has been spent. Reverts
/// if the caller sent less ETH than `amountUsed`.
///
/// Because the caller might not know exactly how much ETH a Vault action will require, they may send extra.
/// Note that this excess value is returned *to the contract caller* (msg.sender). If caller and e.g. swap sender are
/// not the same (because the caller is a relayer for the sender), then it is up to the caller to manage this
/// returned ETH.
pub fn handle_remaining_eth(amountUsed: u64) {
    require(msg_amount() >= amountUsed, Error::INSUFFICIENT_ETH);

    let excess: u64 = msg_amount() - amountUsed;
    if (excess > 0) {
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address, _ => revert(0), 
        };
        transfer_to_output(excess, contract_id(), sender);
        // msg.sender.sendValue(excess);
    }
}

// Returns an ordered pair (amountIn, amountOut) given the 'given' and 'calculated' amounts, and the swap kind.
pub fn get_amounts(
    kind: SwapKind,
    amountGiven: u64,
    amountCalculated: u64,
) -> (u64, u64) {
    if let SwapKind::GIVEN_IN = kind {
        return (amountGiven, amountCalculated);
    } else {
        // SwapKind::GIVEN_OUT
        return (amountCalculated, amountGiven);
    }
}

// Returns the address of a Pool's contract.
// Due to how Pool IDs are created, this is done with no storage accesses and costs little gas.
pub fn get_pool_address(poolId: b256) -> Address {
    // 12 byte logical shift left to remove the nonce and specialization setting. We don't need to mask,
    // since the logical shift already sets the upper bits to zero.
    return ~Address::from(poolId);
}

// Calls the onSwap hook for a Pool that implements IMinimalSwapInfoPool: both Minimal Swap Info and Two Token
// Pools do this.
pub fn call_minimal_swap_info_pool_on_swap_hook(
    request: SwapRequest,
    pool: ContractId,
    tokenInBalance: b256,
    tokenOutBalance: b256
)
    -> (b256, b256, u64)
{   
    let mut request = request;
    // todo need to see this
    let tokenInTotal = total(tokenInBalance);
    let tokenOutTotal = total(tokenOutBalance);
    request.lastChangeBlock = max(last_change_block(tokenInBalance), last_change_block(tokenOutBalance));

    // Perform the swap request callback, and compute the new balances for 'token in' and 'token out' after the swap
    // todo pool-utils/contracts/BaseMinimalSwapInfoPool
    // let amountCalculated = on_swap(pool, request, tokenInTotal, tokenOutTotal);
    let amountCalculated = 0;
    let(amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);

    let newTokenInBalance = increase_cash(tokenInBalance, amountIn);
    let newTokenOutBalance = decrease_cash(tokenOutBalance, amountOut);

    return (newTokenInBalance, newTokenOutBalance, amountCalculated);
}

