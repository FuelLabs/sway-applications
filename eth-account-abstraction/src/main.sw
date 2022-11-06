script;

dep utils;

use utils::{
    eip_191_format,
};

use std::{
    b512::B512,
    hash::sha256,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

/*
// Recover the EVM address from the input signature and check if it matched 
// the target address
fn main(signature: B512, message_hash: b256) -> (u8, u8, b256) {
    //EIP 191 format
    let eip_191_formatted_message = eip_191_format(message_hash);

    //Sha hash
    let recovery_message = sha256(eip_191_formatted_message);

    let target_address = EvmAddress::from(
        0x44c646ac0426710470343f1cdb4aa29ef306fc8d28025b838ccd3feecaedb333
    );

    //recover evm address from signature
    let evm_address_result = ec_recover_evm_address(signature, recovery_message);
    require(evm_address_result.is_ok(),"ec recover evm address failed");
    let evm_address = evm_address_result.unwrap();

    //Check inputs to hash, within eip_191_format
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;
    return (
        initial_byte,
        version_byte,
        message_hash
    );
}
*/

/*
Data passed into Keccak hash in eip-191 formmatting
In rust, with Vec<u8>
[25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
In sway, with (u8,u8.b256). u8s are padded to u64.
[0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
*/

fn main(signature: B512, message_hash: b256) -> ( u64, u64, u64, u64, u64) {
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;

    let packed_bytes = compose(initial_byte, version_byte, 0 ,0);

    let encoded_data = encode_data(packed_bytes, message_hash);

    return (
        encoded_data.get(0).unwrap(),
        encoded_data.get(1).unwrap(),
        encoded_data.get(2).unwrap(),
        encoded_data.get(3).unwrap(),
        encoded_data.get(4).unwrap(),
    );
}

/*
fn pack_bytes(bytes_array: [u8; 8]) -> u64 {
    let mut packed: u64 = 0;
    let mut i = 1;
    while i < 9 {
        packed = packed + (bytes_array[i - 1] << (64 - (i * 8)));
        i += 1;
    };
    packed
}
*/

/// Build a single b256 value from 4 64 bit words.
fn compose(word_1: u64, word_2: u64, word_3: u64, word_4: u64) -> b256 {
    let res: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    asm(w1: word_1, w2: word_2, w3: word_3, w4: word_4, result: res) {
        sw result w1 i0;
        sw result w2 i1;
        sw result w3 i2;
        sw result w4 i3;
        result: b256
    }
}

/// Get 4 64 bit words from a single b256 value.
fn decompose(val: b256) -> (u64, u64, u64, u64) {
    let w1 = get_word_from_b256(val, 0);
    let w2 = get_word_from_b256(val, 8);
    let w3 = get_word_from_b256(val, 16);
    let w4 = get_word_from_b256(val, 24);
    (w1, w2, w3, w4)
}

/// Extract a single 64 bit word from a b256 value using the specified offset.
fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    asm(r1: val, offset: offset, r2, res) {
        add r2 r1 offset;
        lw res r2 i0;
        res: u64
    }
}

/// Encode the packed_bytes and message_hash into a Vec<u64>
fn encode_data(packed_bytes: b256, message_hash: b256) -> Vec<u64> {
    let mut data = Vec::with_capacity(5);
    let (bytes_1, bytes_2, _bytes_3, _bytes_4) = decompose(packed_bytes);
    let (message_1, message_2, message_3, message_4) = decompose(message_hash);

    data.push(
        (bytes_1 << 56) + 
        (bytes_2 << 48) +
        (message_1 >> 16)
        );

    data.push(
        (message_1 << 48) +
        (message_2 >> 16)
    );
    data.push(
        (message_2 << 48) +
        (message_3 >> 16)
    );
    data.push(
        (message_3 << 48) +
        (message_4 >> 16)
    );
    data.push(
        message_4 << 48
    );

    data
}

/*
Rust Script : [25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
Sway script : [25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239, 0, 0, 0, 0, 0, 0]
*/