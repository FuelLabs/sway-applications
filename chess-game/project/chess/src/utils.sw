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

// used to generate a mask for clearing a nibble, eg:
// 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0FFFFFFFFFFFFFFFFFFFF;
pub fn b256_multimask(n: u64) -> b256 {
    let LSB_ONES: b256 = 0x0000000000000000000000000000000000000000000000001111111111111111;
    let mut mask_part_1 = ZERO_B256;
    let mut mask_part_2 = ZERO_B256;
    assert(n < 64);
    let mut nibble_index = n * 4;

    let mut left_mask_size = 256 - nibble_index - 4;
    let words_left_of_target = left_mask_size / 64;
    let remainder_l = left_mask_size % 64;
    let remainder_l_mask = compose((0, 0, 0, remainder_l));

    let words_right_of_target = nibble_index / 4;
    let remainder_r = nibble_index % 64;
    let remainder_r_mask = compose((0, 0, 0, remainder_r));

    let mut i = 0;
    while i < words_left_of_target {
        let shift = (i + 1) * 64;
        mask_part_1 = mask_part_1 | (LSB_ONES << 256 - shift);
        i += 1;
    };
    mask_part_1 = mask_part_1 | (remainder_l_mask << ((words_right_of_target * 64) + 64 - remainder_l));

    i = 0;
    while i < words_right_of_target {
        let shift = (i + 1) * 64;
        mask_part_2 = mask_part_2 | (LSB_ONES << shift);
        i += 1;
    };
    mask_part_2 = mask_part_2 | (remainder_r_mask << words_right_of_target * 64);

    mask_part_1 | mask_part_2
}

/// Get a bitmask with a single `1` at the nth position.
pub fn single_bit_mask(n: u64) -> u64 {
    // TODO: fix bug ! when n == 0
    1 << n
}


