library utils;

dep data_structures;

use std::{
    call_frames::contract_id,
    ecr::{
        ec_recover_address,
    },
    hash::{
        sha256,
    },
    identity::Identity,
    vm::evm::{
        ecr::ec_recover_evm_address,
    },
};

use data_structures::{MessageFormat, MessagePrefix, SignatureData, Transaction, WalletType};

/// Takes in transaction data and hashes it into a unique transaction hash.
pub fn create_hash(to: Identity, value: u64, data: b256, nonce: u64) -> b256 {
    sha256(Transaction {
        contract_identifier: contract_id(),
        data,
        destination: to,
        nonce,
        value,
    })
}

/// Applies the format and prefix specified by signature_data to the message_hash.
/// Then recovers to the relevant address type specified by signature_data.
/// Returns the b256 value of the recovered address.
pub fn recover_signer(message_hash: b256, signature_data: SignatureData) -> b256 {
    let formatted_message = match signature_data.format {
        MessageFormat::None => message_hash,
        MessageFormat::EIP191PersonalSign => eip_191_personal_sign_format(message_hash),
    };

    let prefixed_message = match signature_data.prefix {
        MessagePrefix::None => formatted_message,
        MessagePrefix::Ethereum => ethereum_prefix(formatted_message),
    };

    match signature_data.wallet_type {
        WalletType::Fuel => {
            let recover_result = ec_recover_address(signature_data.signature, prefixed_message);
            require(recover_result.is_ok(), "ec_recover_address failed");
            recover_result.unwrap().value
        },
        WalletType::EVM => {
            let recover_result = ec_recover_evm_address(signature_data.signature, prefixed_message);
            require(recover_result.is_ok(), "ec_recover_evm_address failed");
            recover_result.unwrap().value
        },
    }
}

/// Creates an EIP-191 compliant transaction hash, of the version:
/// 0x45, personal sign.
/// It takes a data_to_sign to represent the <data to sign> in the following EIP-191 format:
/// 0x19 <1 byte version> <version specific data> <data to sign>
fn eip_191_personal_sign_format(data_to_sign: b256) -> b256 {
    let initial_byte = 0x19u8;
    let version_byte = 0x45u8;

    let encoded_data = encode_data(initial_byte, version_byte, data_to_sign);
    let encoded_data = (
        encoded_data.get(0).unwrap(),
        encoded_data.get(1).unwrap(),
        encoded_data.get(2).unwrap(),
        encoded_data.get(3).unwrap(),
        encoded_data.get(4).unwrap(),
    );

    // Keccak256 hash the first 34 bytes of encoded_data
    let mut result_buffer: b256 = b256::min();
    asm(hash: result_buffer, ptr: encoded_data, bytes: 34) {
        k256 hash ptr bytes;
        hash: b256
    }
}

/// Encode the packed_bytes, version_byte and message_hash into a Vec<u64> of length 40 bytes,
/// where the first 34 bytes are the desired data tightly packed.
fn encode_data(initial_byte: u64, version_byte: u64, message_hash: b256) -> Vec<u64> {
    let mut data = Vec::with_capacity(5);

    let (message_1, message_2, message_3, message_4) = decompose(message_hash);

    data.push((initial_byte << 56) + (version_byte << 48) + (message_1 >> 16));

    data.push((message_1 << 48) + (message_2 >> 16));
    data.push((message_2 << 48) + (message_3 >> 16));
    data.push((message_3 << 48) + (message_4 >> 16));
    data.push(message_4 << 48);

    data
}

/// Get a tuple of 4 u64 values from a single b256 value.
fn decompose(val: b256) -> (u64, u64, u64, u64) {
    asm(r1: __addr_of(val)) { r1: (u64, u64, u64, u64) }
}

/// Applies the prefix used by Geth to a message hash.
/// Returns the prefixed hash.
fn ethereum_prefix(msg_hash: b256) -> b256 {
    let prefix = "\x19Ethereum Signed Message:\n32";

    sha256((prefix, msg_hash))
}
