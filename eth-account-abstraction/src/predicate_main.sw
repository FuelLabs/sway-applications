predicate;

dep utils;

use std::{
    b512::B512, 
    constants::ZERO_B256, 
    ecr::ec_recover_address, 
    inputs::input_predicate_data,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

fn recover_and_match(signature: B512, expected_address: b256) -> u64 {
    // if let Result::Ok(address) = ec_recover_address(signature, ZERO_B256)
    if let Result::Ok(address) = ec_recover_evm_address(signature, ZERO_B256)
    {
        if address.value == expected_address {
            return 1;
        }
    }
    0
}

fn main() -> bool {
    let signature: [B512; 1] = input_predicate_data(0);

    let spender_address = [
        // 0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1,//fuel address
        // 0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333//evm address
    ];

    let mut matched_addresses = 0;

    matched_addresses = recover_and_match(signature[0], spender_address[0]);

    matched_addresses > 0
}