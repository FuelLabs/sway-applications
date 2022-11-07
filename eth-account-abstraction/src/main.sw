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


// Recover the EVM address from the input signature and check if it matched 
// the target address
fn main(signature: B512, message_hash: b256) -> 
    //(u8, u8, b256) 
    bool
    {
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

    
    evm_address == target_address
    

    /*
    //
    //Check inputs to hash, within eip_191_format
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;
    return (
        initial_byte,
        version_byte,
        message_hash
    );
    */
}

/*
Rust Script : [25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
Sway script : [0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
*/

fn bitshift_compare(signature: B512, message_hash: b256) -> ( u64, u64, u64, u64, u64) {
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;

    let packed_bytes = compose((initial_byte, version_byte, 0 ,0));

    let encoded_data = encode_data(packed_bytes, message_hash);

    return (
        encoded_data.get(0).unwrap(),
        encoded_data.get(1).unwrap(),
        encoded_data.get(2).unwrap(),
        encoded_data.get(3).unwrap(),
        encoded_data.get(4).unwrap(),
    );
}

/// Build a single b256 value from a tuple of 4 u64 values.
pub fn compose(words: (u64, u64, u64, u64)) -> b256 {
    asm(r1: __addr_of(words)) { r1: b256 }
}

/// Get a tuple of 4 u64 values from a single b256 value.
pub fn decompose(val: b256) -> (u64, u64, u64, u64) {
    asm(r1: __addr_of(val)) { r1: (u64, u64, u64, u64) }
}

/// Encode the packed_bytes and message_hash into a Vec<u64>
/*
Bitshifts:
[0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
Into:
[25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239, 0, 0, 0, 0, 0, 0]

*/
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