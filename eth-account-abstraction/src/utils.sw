library utils;

use std::{
    b512::B512,
    ecr::{ec_recover, EcRecoverError},
    hash::{
        keccak256,
        sha256,
    },
    identity::Identity,
    vm::evm::evm_address::EvmAddress,
};

//Applies the prefix used by Geth to a message hash.
//Returns the prefixed hash.
pub fn eth_prefix(msg_hash: b256) -> b256 {
    let prefix = "\x19Ethereum Signed Message:\n32";
    sha256((prefix, msg_hash))
}

//Creates an EIP-191 compliant transaction hash, of the version:
//0x45, personal sign.
//It takes a data_to_sign to represent the <data to sign> in the following EIP-191 format:
//0x19 <1 byte version> <version specific data> <data to sign>
pub fn eip_191_format(
    data_to_sign: b256
) -> b256 {
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;
    keccak256((
        initial_byte,
        version_byte,
        data_to_sign
    ))
}

//Recovers an EVM address from a signature over signed_data that was:
//EIP-191 compliant, 
//and received an Ethereum signed message prefix.
pub fn ec_recover_evm_address (signature: B512, msg_hash: b256) -> Result<EvmAddress, EcRecoverError> {
    let tx_hash = eip_191_format(msg_hash);
    let tx_hash = eth_prefix(tx_hash);

    let pub_key_result = ec_recover(signature, tx_hash);
    match pub_key_result {
        Result::Err(e) => Result::Err(e),
        _ => {
            let pub_key = pub_key_result.unwrap();
            let pubkey_hash = keccak256((
                (pub_key.bytes)[0],
                (pub_key.bytes)[1],
            ));
            Result::Ok(EvmAddress::from(pubkey_hash))
        }
    }
}