library;

use std::{
    call_frames::contract_id,
    ecr::ec_recover_address,
    hash::{
        Hash,
        Hasher,
        keccak256,
        sha256,
    },
    vm::evm::ecr::ec_recover_evm_address,
};

use ::data_structures::{
    hashing::{
        Threshold,
        Transaction,
        TypeToHash,
        Weight,
    },
    signatures::{
        MessageFormat,
        MessagePrefix,
        SignatureInfo,
        WalletType,
    },
    user::User,
};

const EIP191_INITIAL_BYTE = 0x19;
const EIP191_VERSION_BYTE = 0x45;
// const ETHEREUM_PREFIX = "\x19Ethereum Signed Message:\n32"; // TODO: Replace the use of string literal with this constant when compiler bug is fixed.

/// Takes a struct comprised of transaction data and hashes it.
///
/// # Additional Information
///
/// The struct will be a variant of [TypeToHash].
///
/// # Arguments
///
/// * `type_to_hash` : [TypeToHash] - The struct to hash.
///
/// # Returns
///
/// * [b256] - The hash.
pub fn compute_hash(type_to_hash: TypeToHash) -> b256 {
    match type_to_hash {
        TypeToHash::Threshold(threshold) => sha256(threshold),
        TypeToHash::Transaction(transaction) => sha256(transaction.into_bytes()),
        TypeToHash::Weight(weight) => sha256(weight),
    }
}

/// Applies the format and prefix specified by `signature_info` to the `message_hash`.
/// Returns the [b256] value of the recovered address.
///
/// # Arguments
///
/// * `message_hash`: [b256] - The message hash to be formatted and prefixed.
/// * `signature_info`: [SignatureInfo] - The information about a user's signature for a specific transaction.
///
/// # Returns
///
/// * [b256] - The recovered address.
pub fn recover_signer(message_hash: b256, signature_info: SignatureInfo) -> b256 {
    let formatted_message = match signature_info.message_format {
        MessageFormat::None => message_hash,
        MessageFormat::EIP191PersonalSign => eip_191_personal_sign_format(message_hash),
    };

    let prefixed_message = match signature_info.message_prefix {
        MessagePrefix::None => formatted_message,
        MessagePrefix::Ethereum => ethereum_prefix(formatted_message),
    };

    match signature_info.wallet_type {
        WalletType::Fuel => {
            let recover_result = ec_recover_address(signature_info.signature, prefixed_message);
            require(recover_result.is_ok(), recover_result.unwrap());
            recover_result.unwrap().value
        },
        WalletType::EVM => {
            let recover_result = ec_recover_evm_address(signature_info.signature, prefixed_message);
            require(recover_result.is_ok(), recover_result.unwrap());
            recover_result.unwrap().value
        },
    }
}

/// Creates an EIP-191 compliant transaction hash, of the version:
/// 0x45 - personal sign.
/// It takes a `data_to_sign` to represent the <data to sign> in the EIP-191 format:
/// 0x19 <1 byte version> <version specific data> <data to sign>
///
/// # Additional Information
///
/// EIP-191: https://eips.ethereum.org/EIPS/eip-191
///
/// # Arguments
///
/// * `data_to_sign`: [b256] - The message hash to format.
///
/// # Returns
///
/// * [b256] - The formatted message hash.
fn eip_191_personal_sign_format(data_to_sign: b256) -> b256 {
    let signed_data = encode_and_pack_signed_data(EIP191_INITIAL_BYTE, EIP191_VERSION_BYTE, data_to_sign);
    let signed_data = (
        signed_data.get(0).unwrap(),
        signed_data.get(1).unwrap(),
        signed_data.get(2).unwrap(),
        signed_data.get(3).unwrap(),
        signed_data.get(4).unwrap(),
    );

    // Keccak256 hash the first 34 bytes of encoded_data
    let mut result_buffer = b256::min();
    asm(hash: result_buffer, ptr: signed_data, bytes: 34) {
        k256 hash ptr bytes;
        hash: b256
    }
}

/// Encode the `initial_byte`, `version_byte` and `message_hash` into a [Vec<u64>] of length 40 bytes,
/// where the first 34 bytes are the desired `signed_data` tightly packed.
///
/// # Arguments
///
/// * `initial_byte`: [u64] - EIP-191 initial byte.
/// * `version_byte`: [u64] - EIP-191 version byte.
/// * `message_hash`: [b256] - The message hash to encode.
///
/// # Returns
///
/// * [Vec<u64>] - The encoded data, tightly packed.
fn encode_and_pack_signed_data(
    initial_byte: u64,
    version_byte: u64,
    message_hash: b256,
) -> Vec<u64> {
    let mut data = Vec::with_capacity(5);

    // `message_1`, `message_2`, `message_3` and `message_4` are the four `u64`s that made up the `b256` `message_hash`.
    let (message_1, message_2, message_3, message_4) = decompose(message_hash);

    data.push((initial_byte << 56) + (version_byte << 48) + (message_1 >> 16));
    data.push((message_1 << 48) + (message_2 >> 16));
    data.push((message_2 << 48) + (message_3 >> 16));
    data.push((message_3 << 48) + (message_4 >> 16));
    data.push(message_4 << 48);

    data
}

/// Get a tuple of 4 [u64] values from a single [b256] value.
///
/// # Arguments
///
/// * `value`: [b256] - The value to decompose.
///
/// # Returns
///
/// * [(u64, u64, u64, u64)] - The [u64]s that comprised `value`.
fn decompose(value: b256) -> (u64, u64, u64, u64) {
    asm(r1: __addr_of(value)) {
        r1: (u64, u64, u64, u64)
    }
}

/// Applies the prefix "\x19Ethereum Signed Message:\n32" to a message hash.
///
/// # Arguments
///
/// * `msg_hash`: [b256] - The message_hash.
///
/// # Returns
///
/// * [b256]- The prefixed hash.
fn ethereum_prefix(msg_hash: b256) -> b256 {
    keccak256(("\x19Ethereum Signed Message:\n32", msg_hash)) //// TODO: Replace the use of string literal with this constant when compiler bug is fixed.
}
