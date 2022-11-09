script;

dep utils;

use utils::{eip_191_format, eth_prefix};

use std::{
    b512::B512,
    hash::sha256,
    vm::evm::{
        ecr::ec_recover_evm_address,
        evm_address::EvmAddress,
    },
};

// Recover the EVM address from the input signature and message.
// Check if it matched the target address.
fn main(signature: B512, message_hash: b256) -> bool {
    let eip_191_formatted_message = eip_191_format(message_hash);

    let eth_prefixed_message = eth_prefix(eip_191_formatted_message);

    let target_address = EvmAddress::from(0x44c646ac0426710470343f1cdb4aa29ef306fc8d28025b838ccd3feecaedb333);

    //recover evm address from signature
    let evm_address_result = ec_recover_evm_address(signature, eth_prefixed_message);
    require(evm_address_result.is_ok(), "ec recover evm address failed");
    let evm_address = evm_address_result.unwrap();

    evm_address == target_address
}
