library utils;

dep data_structures;
dep errors;

use data_structures::PoolSpecialization;
use errors::Error;

use std::{
    address::Address,
    math::*,
    revert::require,
};

pub fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    let mut empty: u64 = 0;
    asm(r1: val, offset: offset, r2, res: empty) {
        add r2 r1 offset;
        lw res r2 i0;
        res: u64
    }
}

// Returns the address of a Pool's contract.
// Due to how Pool IDs are created, this is done with no storage accesses and costs little gas.
pub fn get_pool_address(poolId: b256) -> Address {
    // 12 byte logical shift left to remove the nonce and specialization setting. We don't need to mask,
    // since the logical shift already sets the upper bits to zero.
    return ~Address::from(poolId);
}

// Creates a Pool ID.
//
// These are deterministically created by packing the Pool's contract address and its specialization setting into
// the ID. This saves gas by making this data easily retrievable from a Pool ID with no storage accesses.
//
// Since a single contract can register multiple Pools, a unique nonce must be provided to ensure Pool IDs are
// unique.
//
// Pool IDs have the following layout:
// | 20 bytes pool contract address | 2 bytes specialization setting | 10 bytes nonce |
// MSB                                                                              LSB
//
// 2 bytes for the specialization setting is a bit overkill: there only three of them, which means two bits would
// suffice. However, there's nothing else of interest to store in this extra space.
//Todo Need workaround for this
pub fn to_pool_id(
    pool: Address,
    specialization: PoolSpecialization,
    nonce: u64
) -> b256 {
    let mut serialized: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    //Todo Need some workaround
    // serialized |= bytes32(uint256(nonce));
    // serialized |= bytes32(uint256(specialization)) << (10 // 8);
    // serialized |= bytes32(uint256(pool)) << (12 // 8);

    return serialized;
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
