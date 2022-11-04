library utils;
dep errors;
dep data_structures;
dep ops;

use errors::Error;
use data_structures::{
    abi_encode,
    FUEL,
    MASK,
    ONE,
    PoolSpecialization,
    SwapKind,
    SwapRequest,
    UserBalanceOp,
    UserBalanceOpKind,
    WFUEL,
};

use ops::{binary_or, compose, get_word_from_b256, lsh, lsh_u64};

use std::{
    address::Address,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    context::{
        call_frames::contract_id,
        msg_amount,
    },
    contract_id::ContractId,
    hash::keccak256,
    identity::Identity,
    math::*,
    option::Option,
    result::*,
    revert::{
        require,
        revert,
    },
    token::{
        force_transfer_to_contract,
        transfer_to_output,
    },
    vec::Vec,
};

pub fn mul_up(a: u64, b: u64) -> u64 {
    let product = a * b;
    require(a == 0 || product / a == b, Error::MUL_OVERFLOW);

    if product == 0 {
        0
    } else {
        // The traditional div_up formula is:
        // div_up(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // div_up(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, which we already tested for.
        let res: u64 = ((product - 1) / ONE) + 1;
        res
    }
}

// Returns true if `asset` is the sentinel value that represents FUEL.
pub fn is_eth(asset: ContractId) -> bool {
    let asset = ~ContractId::from(FUEL);
    return (asset == ~ContractId::from(FUEL));
}

// Translates `asset` into an equivalent IERC20 token address. If `asset` represents FUEL, it will be translated
// to the WFUEL contract.
pub fn translate_to_ierc20(asset: ContractId) -> ContractId {
    if is_eth(asset) {
        return ~ContractId::from(WFUEL);
    }
    return asset;
}

// Same as `_translateToIERC20(IAsset)`, but for an entire array.
pub fn translate_to_ierc20_second(asset: Vec<ContractId>) -> Vec<ContractId> {
    let mut tokens: Vec<ContractId> = ~Vec::new();
    let mut i: u64 = 0;
    while i < asset.len() {
        tokens.push(translate_to_ierc20(asset.get(i).unwrap()));
        i = i + 1;
    }
    return tokens;
}

// For `swap_with_pool` to handle both 'given in' and 'given out' swaps, it internally tracks the 'given' amount
// (supplied by the caller), and the 'calculated' amount (returned by the Pool in response to the swap request).
// Given the two swap tokens and the swap kind, returns which one is the 'given' token (the token whose
// amount is supplied by the caller).
pub fn token_given(kind: SwapKind, tokenIn: ContractId, tokenOut: ContractId, ) -> ContractId {
    if let SwapKind::GIVEN_IN = kind {
        return tokenIn;
    } else {
        return tokenOut;
    }
}

// Returns the specialization setting of a Pool.
// Due to how Pool IDs are created, this is done with no storage accesses and costs little gas.
pub fn get_pool_specialization(poolId: b256) -> PoolSpecialization {
    // 10 byte logical shift left to remove the nonce, followed by a 2 byte mask to remove the address.
    let value = get_word_from_b256((poolId >> (10 * 8)), 32) & (2.pow(2 * 8) - 1);

    // Casting a value into an enum results in a runtime check that reverts unless the value is within the enum's
    // range. Passing an invalid Pool ID to this function would then result in an obscure revert with no reason
    // string: we instead perform the check ourselves to help in error diagnosis.
    // There are three Pool specialization settings: general, minimal swap info and two tokens, which correspond to
    // values 0, 1 and 2.
    require(value < 3, Error::INVALID_POOL_ID);

    // Because we have checked that `value` is within the enum range, we can upoolIdse assembly to skip the runtime check.
    let _value = PoolSpecialization::GENERAL;
    _value
}

// Same as `_translateToIERC20(IAsset)`, but for an entire array.
pub fn translate_to_ierc20_array(asset: Vec<ContractId>) -> Vec<ContractId> {
    let mut tokens: Vec<ContractId> = ~Vec::new();
    let mut i: u64 = 0;
    while i < asset.len() {
        tokens.push(translate_to_ierc20(asset.get(i).unwrap()));
        i = i + 1;
    }
    return tokens;
}

// Given the two swap tokens and the swap kind, returns which one is the 'calculated' token (the token whose
// amount is calculated by the Pool).
pub fn token_calculated(kind: SwapKind, tokenIn: ContractId, tokenOut: ContractId) -> ContractId {
    if let SwapKind::GIVEN_IN = kind {
        return tokenOut;
    } else {
        return tokenIn;
    }
}

pub fn sort_two_tokens(token_x: ContractId, token_y: ContractId) -> (ContractId, ContractId) {
    let token_a: b256 = token_x.into();
    let token_b: b256 = token_y.into();
    if token_a < token_b {
        return (token_x, token_y);
    }
    return (token_y, token_x);
}

pub fn get_two_token_pair_hash(token_a: ContractId, token_b: ContractId) -> b256 {
    let tmp = abi_encode {
        token_a: token_a,
        token_b: token_b,
    };
    return keccak256(tmp);
}

// helping function
pub fn vec_contains(vec: Vec<ContractId>, search: ContractId) -> bool {
    let mut count = 0;
    while (count < vec.len()) {
        if vec.get(count).unwrap() == search {
            return true;
        }
        count = count + 1;
    }

    return false;
}

pub fn vec_contains1(vec: Vec<b256>, search: b256) -> bool {
    let mut count = 0;
    while count < vec.len() {
        if vec.get(count).unwrap() == search {
            return true;
        }
        count = count + 1;
    }

    return false;
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
            Identity::Address(address) => address,
            _ => revert(0),
        };
        transfer_to_output(excess, contract_id(), sender);
        // msg.sender.sendValue(excess);
    }
}

// Returns an ordered pair (amountIn, amountOut) given the 'given' and 'calculated' amounts, and the swap kind.
pub fn get_amounts(kind: SwapKind, amountGiven: u64, amountCalculated: u64, ) -> (u64, u64) {
    if let SwapKind::GIVEN_IN = kind {
        return (
            amountGiven,
            amountCalculated,
        );
    } else {
        // SwapKind::GIVEN_OUT
        return (
            amountCalculated,
            amountGiven,
        );
    }
}

// Returns the address of a Pool's contract.
// Due to how Pool IDs are created, this is done with no storage accesses and costs little gas.
pub fn get_pool_address(poolId: b256) -> ContractId {
    // 12 byte logical shift left to remove the nonce and specialization setting. We don't need to mask,
    // since the logical shift already sets the upper bits to zero.
    // todo
    // address(uint256(poolId) >> (12 * 8));
    return ~ContractId::from(poolId);
}

// Casts an array of uint256 to int256, setting the sign of the result according to the `positive` flag,
// without checking whether the values fit in the signed 256 bit range.
pub fn unsafe_cast_to_int256(values: Vec<u64>, positive: bool) -> Vec<u64> {
    let mut signedValues = ~Vec::new();
    let mut count = 0;
    while count < values.len() {
        if positive {
            // signedValues.push(-values.get(count).unwrap());
            signedValues.push(values.get(count).unwrap());
        } else {
            signedValues.push(values.get(count).unwrap());
        }
        count = count + 1;
    }
    return signedValues;
}

/// Destructures a User Balance operation, validating that the contract caller is allowed to perform it.
pub fn validate_user_balance_op(
    op: UserBalanceOp,
    checkedCallerIsRelayer: bool,
) -> (UserBalanceOpKind, ContractId, u64, Address, Address, bool) {
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

    return (
        op.kind,
        op.asset,
        op.amount,
        sender,
        op.recipient,
        tmp,
    );
}

//Todo need to check this again
pub fn to_pool_id(
    pool: Address,
    specialization: PoolSpecialization,
    nonce: u64,
) -> b256 {
    let pool: b256 = pool.into();
    let mut specialization_value = 0;
    if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        specialization_value = 1;
    } else if let PoolSpecialization::TWO_TOKEN = specialization {
        specialization_value = 2;
    }

    let mut serialized: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    serialized = binary_or(serialized, compose(nonce, 0, 0, 0, ));
    serialized = binary_or(serialized, lsh(compose(specialization_value, 0, 0, 0, ), 80));
    serialized = binary_or(serialized, lsh(pool, 96));

    return serialized;
}

// ! BalanceAllocation
pub fn last_change_block(balance: b256) -> u64 {
    // let mask: u64 = 2**(32) - 1;
    return ((get_word_from_b256(balance, 0) >> 224) & MASK);
}

pub fn increase_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) + amount;
    let current_managed: u64 = managed(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(new_cash, current_managed, new_last_change_block);
}

pub fn decrease_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) - amount;
    let current_managed: u64 = managed(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(new_cash, current_managed, new_last_change_block);
}

pub fn total(balance: b256) -> u64 {
    // return cash(balance) + managed(balance);
    return (cash(balance) + managed(balance));
}

pub fn is_zero(balance: b256) -> bool {
    // let mask: u64 = 2**(224) - 1;
    return (get_word_from_b256(balance, 0) & MASK) == 0;
}

pub fn from_shared_to_balance_a(shared_cash: b256, shared_managed: b256) -> b256 {
    return to_balance(decode_balance_a(shared_cash), decode_balance_a(shared_managed), last_change_block(shared_cash));
}

pub fn from_shared_to_balance_b(shared_cash: b256, shared_managed: b256) -> b256 {
    return to_balance(decode_balance_b(shared_cash), decode_balance_b(shared_managed), last_change_block(shared_cash));
}

pub fn to_shared_cash(token_a_balance: b256, token_b_balance: b256) -> b256 {
    let new_last_change_block: u64 = max(last_change_block(token_a_balance), last_change_block(token_b_balance));

    return pack(cash(token_a_balance), cash(token_b_balance), new_last_change_block);
}

// change/doubts -> we might need to use `insert` method on vec, instead of push
pub fn totals_and_last_change_block(balances: Vec<b256>) -> (Vec<u64>, u64) {
    let mut i = 0;
    let mut results = ~Vec::new();
    let mut last_change_block_time = 0;

    while (i < results.len()) {
        let balance = balances.get(i).unwrap();
        results.push(total(balance));
        // results.insert(i, total(balance));
        last_change_block_time = max(last_change_block_time, last_change_block(balance));

        i += 1;
    }

    return (
        results,
        last_change_block_time,
    );
}

pub fn cash_to_managed(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) - amount;
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_managed: u64 = managed(balance) + amount;
    let current_last_change_block: u64 = last_change_block(balance);

    return to_balance(new_cash, new_managed, current_last_change_block);
}

pub fn managed_delta(new_balance: b256, old_balance: b256) -> u64 {
    return (managed(new_balance) - managed(old_balance));
}

pub fn to_shared_managed(token_a_balance: b256, token_b_balance: b256) -> b256 {
    return pack(managed(token_a_balance), managed(token_b_balance), 0);
}

pub fn managed_to_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) + amount;
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_managed: u64 = managed(balance) - amount;
    let current_last_change_block: u64 = last_change_block(balance);

    return to_balance(new_cash, new_managed, current_last_change_block);
}

pub fn set_managed(balance: b256, new_managed: u64) -> b256 {
    let current_cash: u64 = cash(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(current_cash, new_managed, new_last_change_block);
}

pub fn max(first: u64, second: u64) -> u64 {
    if first > second {
        return first;
    } else {
        return second;
    }
}

pub fn cash(balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;
    return (get_word_from_b256(balance, 0) & MASK);
}

pub fn managed(balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;
    return ((get_word_from_b256(balance, 0) >> 112) & MASK);
}

fn to_balance(_cash: u64, _managed: u64, _block_number: u64) -> b256 {
    let _total: u64 = _cash + _managed;

    // mask here -> let mask: u64 = 2**112;
    require(_total >= _cash && _total < MASK, Error::BALANCE_TOTAL_OVERFLOW);

    return pack(_cash, _managed, _block_number);
}

// todo need to check again
fn pack(
    least_significant: u64,
    mid_significant: u64,
    most_significant: u64,
) -> b256 {
    let total = lsh_u64(most_significant, 224) + lsh_u64(mid_significant, 112) + least_significant;
    return (compose(total, 0, 0, 0));
}

fn decode_balance_a(shared_balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;
    return (get_word_from_b256(shared_balance, 0) & MASK);
}

fn decode_balance_b(shared_balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;
    return ((get_word_from_b256(shared_balance, 0) >> 112) & MASK);
}
