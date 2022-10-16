predicate;

use std::{
    address::Address,
    b512::B512,
    constants::ZERO_B256,
    inputs::input_predicate_data,
    vm::evm::{
        ecr::ec_recover_evm_address,
        evm_address::EvmAddress
    },
};

fn main() -> bool {

    //Fuel address derived from known EVM address
    // let spender = ~Address::from(config_spender);
    let spender = ~Address::from(ZERO_B256);

    let signature: B512 = input_predicate_data(0);

    //Derive an EVM address from the signature
    let evm_address_result = ec_recover_evm_address(signature, ZERO_B256);
    require(evm_address_result.is_ok(), "Failed to recover EVM address");
    let evm_address = evm_address_result.unwrap();

    //Derive the Fuel address associated with the EVM address
    let fuel_address = ~Address::from(evm_address.into());

    fuel_address == spender
}