script;

use std::{
    address::Address,
    b512::B512, 
    constants::ZERO_B256, 
    ecr::{
        ec_recover,
        ec_recover_address,
    },
    hash::{
        keccak256,
        sha256,
    },
    inputs::input_predicate_data,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

// fn main(signature: B512) -> (B512, B512, b256, b256) {
//     let message_hash = ZERO_B256;

//     //pK
    // let public_key_result = ec_recover(signature, message_hash);
    // require(public_key_result.is_ok(),"ec recover failed");
    // let public_key = public_key_result.unwrap();

//     //fuel address
//     let fuel_address_result = ec_recover_address(signature, message_hash);
//     require(fuel_address_result.is_ok(),"ec recover address failed");
//     let fuel_address = fuel_address_result.unwrap();

//     //evm address
//     let evm_address_result = ec_recover_evm_address(signature, message_hash);
//     require(evm_address_result.is_ok(),"ec recover evm address failed");
//     let evm_address = evm_address_result.unwrap();

//     //return
//     (
//         signature,
//         public_key,
//         fuel_address.value,
//         evm_address.value,
//     )
// }






//Verify hashing with correct public key
// fn main() -> (B512, b256, b256) {
//     //checking address hashing
//     let derived_pub_key = ~B512::from(
//         0x5a76336abf4ac1b759390876d152e016e7e491dac6b074a1f0cde2caf86c654d,
//         0x699e549564eab8f336589d9664b937394ff1a0037f037068577d1bbbf0e56511
//     );

//     //fuel address
//     let fuel_address = sha256(((derived_pub_key.bytes)[0], (derived_pub_key.bytes)[1]));

//     //evm address
//     let evm_address = keccak256(((derived_pub_key.bytes)[0], (derived_pub_key.bytes)[1]));

//     (
//         derived_pub_key,
//         fuel_address,
//         evm_address
//     )
// }

//Verify ec_recover from correct signature
fn main() -> (B512, B512) {
    //checking address hashing
    let signature = ~B512::from(
        0x4bcc7daf3f607a57dc843c47d97ce580ed3c717a2984938195c1b3d40a5fc580,
        0x135122329c2f8f636bddc939bd838a2c2fc43082d9b59f93ea7cb75e01de59b7
    );

    let public_key_result = ec_recover(signature, ZERO_B256);
    require(public_key_result.is_ok(),"ec recover failed");
    let public_key = public_key_result.unwrap();

    (
        signature,
        public_key
    )
}