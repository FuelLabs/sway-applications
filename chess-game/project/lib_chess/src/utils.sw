library utils;

use std::constants::ZERO_B256;

/// Build a single b256 value from a tuple of 4 u64 values.
pub fn compose(words: (u64, u64, u64, u64)) -> b256 {
    asm(r1: __addr_of(words)) { r1: b256 }
}
/// Get a tuple of 4 u64 values from a single b256 value.
pub fn decompose(val: b256) -> (u64, u64, u64, u64) {
    asm(r1: __addr_of(val)) { r1: (u64, u64, u64, u64) }
}

// Bitwise operations helpers
const MAX_BINARY_U64: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;
const MAX_B256: b256 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

/// Set the nth bit of a bitmap to `0`.
pub fn turn_off_bit(bitmap: u64, n: u64) -> u64 {
    let mask = toggle_bit(MAX_BINARY_U64, n);
    bitmap & mask
}

/// Set the nth bit of a bitmap to `1`.
pub fn turn_on_bit(bitmap: u64, n: u64) -> u64 {
    let mask = toggle_bit(0u64, n);
    bitmap & mask
}

/// Flip the nth bit in a bitmap.
pub fn toggle_bit(bitmap: u64, n: u64) -> u64 {
    let mask = single_bit_mask(n);
    bitmap ^ mask
}

/// Query a bitmat for the state of the nth bit.
pub fn query_bit(bitmap: u64, n: u64) -> u64 {
    let mask = single_bit_mask(n);
    bitmap & mask
}

/// Set the nth bit of a bitmap to `value`
pub fn set_bit(bitmap: u64, n: u64, value: u64) -> u64 {
    let clearing_mask = turn_off_bit(MAX_BINARY_U64, n);
    let new_bitmap = bitmap & clearing_mask;
    let setting_mask = if value == 0 {
        clearing_mask
    } else {
        toggle_bit(0u64, n)
    };
    bitmap | setting_mask
}

/// Get a bitmask of `n` ones.
pub fn multi_bit_mask(n: u64) -> u64 {
    (1 << n) - 1
}

// used to generate a mask for clearing a nibble, eg: b256_multimask(3) returns:
// 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0FFF;
pub fn b256_multimask(n: u64) -> b256 {
    assert(n < 64);
    let mut mask_part_1 = MAX_B256;
    let mut mask_part_2 = MAX_B256;
    mask_part_1 = mask_part_1 << (n * 4 + 4);
    mask_part_2 = mask_part_2 >> (256 - n * 4);
    mask_part_1 | mask_part_2
}

// pub fn b256_set_bit()

/// Get a bitmask with a single `1` at the nth position.
pub fn single_bit_mask(n: u64) -> u64 {
    // TODO: fix bug ! when n == 0
    1 << n
}

#[test()]
fn test_multimask() {
    let m0 = b256_multimask(0);
    let m1 = b256_multimask(1);
    let m11 = b256_multimask(11);
    let m42 = b256_multimask(42);
    let m63 = b256_multimask(63);

    assert(m0 == 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0);
    assert(m1 == 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0F);
    assert(m11 == 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0FFFFFFFFFFF);
    assert(m42 == 0xFFFFFFFFFFFFFFFFFFFFF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    assert(m63 == 0x0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
}


