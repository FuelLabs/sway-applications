script;

use std::{
    address::Address,
    b512::B512, 
    constants::ZERO_B256, 
    ecr::{
        ec_recover,
        ec_recover_address,
    },
    inputs::input_predicate_data,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

fn main(signature: B512) -> (B512, B512, b256, b256) {
    // let signature = c8b46627ad739aecd3e9008ad97722e4ff4a1b3856626778a35cad3369934e6e8a36611b0d1b07be01ca62948b494c50c0abe8dfa4892cea704c83871bae3e02;
    let message_hash = ZERO_B256;

    //pK
    let public_key_result = ec_recover(signature, message_hash);
    require(public_key_result.is_ok(),"ec recover failed");
    let public_key = public_key_result.unwrap();

    //fuel address
    let fuel_address_result = ec_recover_address(signature, message_hash);
    require(fuel_address_result.is_ok(),"ec recover address failed");
    let fuel_address = fuel_address_result.unwrap();

    //evm address
    let evm_address_result = ec_recover_evm_address(signature, message_hash);
    require(evm_address_result.is_ok(),"ec recover evm address failed");
    let evm_address = evm_address_result.unwrap();

    //return
    (
        signature,
        public_key,
        fuel_address.value,
        evm_address.value,
    )
}