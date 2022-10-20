predicate;

use std::{
    b512::B512,
    inputs::input_predicate_data,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

fn main() -> bool {
    let spender_address = ~EvmAddress::from(config_spender);

    let signature: B512 = input_predicate_data(0);
    let predicate_root: b256 = input_predicate_data(1);

    let evm_address_result = ec_recover_evm_address(signature, predicate_root);
    require(evm_address_result.is_ok(), "Unable to recover address");
    let evm_address = evm_address_result.unwrap();

    spender_address == evm_address
}