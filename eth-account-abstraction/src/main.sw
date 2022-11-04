script;

use std::{
    b512::B512, 
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

// Recover the EVM address from the input signature and check if it matched 
// the target address
fn main(signature: B512, message: b256) -> bool {
    let target_address = EvmAddress::from(
        0x44c646ac0426710470343f1cdb4aa29ef306fc8d28025b838ccd3feecaedb333
    );

    //recover evm address from signature
    let evm_address_result = ec_recover_evm_address(signature, message);
    require(evm_address_result.is_ok(),"ec recover evm address failed");
    let evm_address = evm_address_result.unwrap();

    evm_address == target_address
}