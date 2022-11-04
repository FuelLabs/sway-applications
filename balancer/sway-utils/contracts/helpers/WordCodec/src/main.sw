library WordCodec;
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use BalancerErrors::*;
use math::*;
use std ::{
    assert::assert,
    u128::U128,
    result::*,
    revert::{revert, require},
    math::*,
    hash::*,
};
use core::*;

/*
* Library for encoding and decoding values stored inside a 256 bit word. Typically used to pack multiple values in
* a single storage slot, saving gas by performing less storage accesses.
*
* Each value is defined by its size and the least significant bit in the word, also known as offset. For example, two
* 128 bit values may be encoded in a word by assigning one an offset of 0, and the other an offset of 128.
*
* We could use Solidity structs to pack values together in a single storage slot instead of relying on a custom and
* error-prone library, but unfortunately Solidity only allows for structs to live in either storage, calldata or
* memory. Because a memory struct uses not just memory but also a slot in the stack (to store its memory location),
* using memory for word-sized values (i.e. of 256 bits or less) is strictly less gas performant, and doesn't even
* prevent stack-too-deep issues. This is compounded by the fact that Balancer contracts typically are memory-intensive,
* and the cost of accesing memory increases quadratically with the number of allocated words. Manual packing and
* unpacking is therefore the preferred approach.
*/

// Masks are values with the least significant N bits set. They can be used to extract an encoded value from a word,
// or to insert a new one replacing the old.
const _MASK_1: u64 = 1;
const _MASK_192: u64 = 2.pow(192) - 1;

// In-place insertion

/// Extract a single 64 bit word from a b256 value using the specified offset.
pub fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    let mut empty: u64 = 0;
    asm(r1: val, offset: offset, r2, res: empty) {
        add r2 r1 offset;
        lw res r2 i0;
        res: u64
    }
}
/*
    *Inserts an unsigned integer of bitLength, shifted by an offset, into a 256 bit word,
    * replacing the old value. Returns the new word.
*/
pub fn insert_uint(
    word: b256,
    value: u64,
    offset: u64,
    bitLength: u64
) -> b256 {
    _validate_encoding_params(value, offset, bitLength);

    let mut mask: u64 = (1 << bitLength) - 1;
    let clearedWord: b256 = sha256(get_word_from_b256(word,64) & (mask << offset));
    return clearedWord | sha256(value << offset);
}

/*
    * Inserts a signed integer shifted by an offset into a 256 bit word, replacing the old value. Returns
    * the new word.
    *
    * Assumes `value` can be represented using `bitLength` bits.
    */
    // Todo when Signed Ints are added
// pub fn insert_int(
//     word: b256,
//     value: i64,
//     offset: u64,
//     bitLength: u64
// )-> b256  {
//     _validate_encoding_params(value, offset, bitLength);

//     let mask: u64 = (1 << bitLength) - 1;
//     let clearedWord: b256 = bytes32(uint256(word) & ~(mask << offset));
//     // Integer values need masking to remove the upper bits of negative values.
//     return clearedWord | bytes32((uint256(value) & mask) << offset);
// }

// Encoding

/*
    * Encodes an unsigned integer shifted by an offset. Ensures value fits within
    * `bitLength` bits.
    *
    * The return value can be logically ORed with other encoded values to form a 256 bit word.
    */
pub fn encode_uint(
    value: u64,
    offset: u64,
    bitLength: u64
) -> b256 {
    _validate_encoding_params(value, offset, bitLength);
    return sha256(value << offset);
}
    // Todo when Signed Ints are added
/*
    *Encodes a signed integer shifted by an offset.
    *
    * The return value can be logically ORed with other encoded values to form a 256 bit word.
    */
// pub fn encode_int(
//     value: i64,
//     offset: u64,
//     bitLength: u64
// )-> b256 {
//     _validateEncodingParams(value, offset, bitLength);

//     let mask: u64 = (1 << bitLength) - 1;
//     // Integer values need masking to remove the upper bits of negative values.
//     return bytes32((uint256(value) & mask) << offset);
// }

// Decoding

/*
    *Decodes and returns an unsigned integer with `bitLength` bits, shifted by an offset, from a 256 bit word.
    */
pub fn decode_uint(
    word: b256,
    offset: u64,
    bitLength: u64
) -> u64 {
    let _word: u64 = get_word_from_b256(word, 64);
    return (_word >> offset) & ((1 << bitLength) - 1);
}
    // Todo when Signed Ints are added
/*
    * Decodes and returns a signed integer with `bitLength` bits, shifted by an offset, from a 256 bit word.
    */
// pub fn decode_int(
//     word: b256,
//     offset: u64,
//     bitLength: u64
// ) internal pure returns (int256) {
//     let maxInt = int256((1 << (bitLength - 1)) - 1);
//     let mask = (1 << bitLength) - 1;

//     let value = int256(uint256(word >> offset) & mask);
//     // In case the decoded value is greater than the max positive integer that can be represented with bitLength
//     // bits, we know it was originally a negative integer. Therefore, we mask it to restore the sign in the 256 bit
//     // representation.
//     return value > maxInt ? (value | int256(~mask)) : value;
// }

// Special cases

/*
    * Decodes and returns a boolean shifted by an offset from a 256 bit word.
    */
pub fn decode_bool(word: b256, offset: u64) ->bool {
    let mut _word = get_word_from_b256(word, 64);
    return ((_word >> offset) & _MASK_1) == 1;

}

/*
    * Inserts a 192 bit value shifted by an offset into a 256 bit word, replacing the old value.
    * Returns the new word.
    *
    * Assumes `value` can be represented using 192 bits.
    */
pub fn insert_bits192(
    word: b256,
    value: b256,
    offset: u64
) -> b256 {
    let mut a = (_MASK_192 << (offset));
    let clearedWord: b256 = sha256(get_word_from_b256(word,64) & (_MASK_192 << (offset)));
    return clearedWord | sha256((get_word_from_b256(value,64) & _MASK_192) << offset);
}

/*
    * Inserts a boolean value shifted by an offset into a 256 bit word, replacing the old value. Returns the new
    * word.
    */
pub fn insert_bool(
    word: b256,
    value: bool,
    offset: u64
) -> b256 {

    let clearedWord: b256 = sha256(get_word_from_b256(word,64) & (_MASK_1 << (offset)));
    return clearedWord | sha256(get_word_from_b256(
        if value {
            return 1
        }else{
            return 0;
        },64) << offset);
}


pub fn _validate_encoding_params(
    value: u64,
    offset: u64,
    bitLength: u64
) {
    require(offset < 256, OUT_OF_BOUNDS);
    // We never accept 256 bit values (which would make the codec pointless), and the larger the offset the smaller
    // the maximum bit length.
    require(bitLength >= 1 && bitLength <= min(255, 256 - offset), OUT_OF_BOUNDS);

    // Testing unsigned values for size is straightforward: their upper bits must be cleared.
    require(value >> bitLength == 0, CODEC_OVERFLOW);
}

// Todo when Signed Ints are added
// pub fn _validateEncodingParams(
//      value: i64,
//      offset: u64,
//      bitLength: u64
// ) {
//     require(offset < 256, Errors.OUT_OF_BOUNDS);
//     // We never accept 256 bit values (which would make the codec pointless), and the larger the offset the smaller
//     // the maximum bit length.
//     require(bitLength >= 1 && bitLength <= Math.min(255, 256 - offset), Errors.OUT_OF_BOUNDS);

//     // Testing signed values for size is a bit more involved.
//     if (value >= 0) {
//         // For positive values, we can simply check that the upper bits are clear. Notice we remove one bit from the
//         // length for the sign bit.
//         require(value >> (bitLength - 1) == 0, Errors.CODEC_OVERFLOW);
//     } else {
//         // Negative values can receive the same treatment by making them positive, with the caveat that the range
//         // for negative values in two's complement supports one more value than for the positive case.
//         require(Math.abs(value + 1) >> (bitLength - 1) == 0, Errors.CODEC_OVERFLOW);
//     }
// }

