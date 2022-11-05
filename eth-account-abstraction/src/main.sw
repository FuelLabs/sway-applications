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
/*
Data passed into Keccak hash in eip-191 formmatting
In rust, with Vec<u8>
[25, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
In sway, with (u8,u8.b256). u8s are padded to u64.
[0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239]
*/