library utils;

dep data_structures;
dep errors;

use data_structures::{
    UserBalanceOp, 
    UserBalanceOpKind,
    PoolSpecialization,
};
use errors::Error;

use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::{revert, require},
    token::transfer_to_output,
    option::Option,
    math::*,
    vec::Vec,
};


pub fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    let mut empty: u64 = 0;
    asm(r1: val, offset: offset, r2, res: empty) {
        add r2 r1 offset;
        lw res r2 i0;
        res: u64
    }
}

// Returns the specialization setting of a Pool.
// Due to how Pool IDs are created, this is done with no storage accesses and costs little gas.
pub fn get_pool_specialization(poolId: b256) -> PoolSpecialization {
    // 10 byte logical shift left to remove the nonce, followed by a 2 byte mask to remove the address.
    let value = get_word_from_b256((poolId >> (10 * 8)),32) & (2.pow(2 * 8) - 1);

    // Casting a value into an enum results in a runtime check that reverts unless the value is within the enum's
    // range. Passing an invalid Pool ID to this function would then result in an obscure revert with no reason
    // string: we instead perform the check ourselves to help in error diagnosis.

    // There are three Pool specialization settings: general, minimal swap info and two tokens, which correspond to
    // values 0, 1 and 2.
    require(value < 3, Error::INVALID_POOL_ID);

    // Because we have checked that `value` is within the enum range, we can use assembly to skip the runtime check.
    let _value = PoolSpecialization::GENERAL;
    _value
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

/// Destructures a User Balance operation, validating that the contract caller is allowed to perform it.
pub fn validate_user_balance_op(op: UserBalanceOp, checkedCallerIsRelayer: bool)
    -> (
        UserBalanceOpKind,
        ContractId,
        u64,
        Address,
        Address,
        bool
    )
{   
    let mut tmp = checkedCallerIsRelayer;
    // The only argument we need to validate is `sender`, which can only be either the contract caller, or a
    // relayer approved by `sender`.
    let address: Result<Identity, AuthError> = msg_sender();
    let address: Address = match address.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };

    let sender = op.sender;
    if (sender != address) {
        // We need to check both that the contract caller is a relayer, and that `sender` approved them.

        // Because the relayer check is global (i.e. independent of `sender`), we cache that result and skip it for
        // other operations in this same transaction (if any).
        if (!tmp) {
            // todo need msg.sig
            // authenticateCaller();
            tmp = true;
        }

        // require(has_Approved_Relayer(sender, msg_sender), Error::USER_DOESNT_ALLOW_RELAYER);
    }

    return (op.kind, op.asset, op.amount, sender, op.recipient, tmp);
}

// Returns the total balance for `poolId`'s `expectedTokens`.
// `expectedTokens` must exactly equal the token array returned by `getPoolTokens`: both arrays must have the same
// length, elements and order. Additionally, the Pool must have at least one registered token.
pub fn _validate_tokens_and_get_balances(poolId: b256, expectedTokens: Vec<ContractId>) -> Vec<b256> {
    // let x = abi(PoolTokens, pool_tokens_contract_id);
    // let(actualTokens, balances) = x.get_pool_tokens(poolId);
    // ensure_input_length_match(actualTokens.len(), expectedTokens.len());
    // require(actualTokens.len() > 0, "POOL_NO_TOKENS");

    // let mut count = 0;
    // while count < actualTokens.len() {
    //     require(actualTokens.get(count).unwrap() == expectedTokens.get(count).unwrap(), "TOKENS_MISMATCH");
    //     count = count + 1;
    // }
//  
    // todo dummy for now
    let balances: Vec<b256> = ~Vec::new();
    return balances;
}

// Casts an array of uint256 to int256, setting the sign of the result according to the `positive` flag,
// without checking whether the values fit in the signed 256 bit range.
pub fn _unsafe_cast_to_int256(values: Vec<u64>, positive: bool) -> Vec<u64> {
    let mut signedValues = ~Vec::new();
    let mut count = 0;
    while count < values.len() {
        if positive {
            // signedValues.push(-values.get(count).unwrap());
            signedValues.push(values.get(count).unwrap());
        }
        else {
            signedValues.push(values.get(count).unwrap());
        }
        count = count + 1;
    }
    return signedValues;
}
