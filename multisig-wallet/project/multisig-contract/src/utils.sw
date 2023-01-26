library utils;

dep data_structures;

use std::{
    call_frames::contract_id,
    bytes::Bytes,
    ecr::ec_recover_address,
    hash::{
        keccak256,
        sha256,
    },
    vm::evm::ecr::ec_recover_evm_address,
};

use data_structures::{MessageFormat, MessagePrefix, SignatureInfo, Transaction, WalletType};

const EIP191_INITIAL_BYTE = 0x19u8;
const EIP191_VERSION_BYTE = 0x45u8;
const ETHEREUM_PREFIX = "\x19Ethereum Signed Message:\n32";

/// Takes in transaction data and hashes it into a unique transaction hash.
pub fn create_hash(data: Bytes, nonce: u64, to: Identity, value: u64) -> b256 {
    sha256(Transaction {
        contract_identifier: contract_id(),
        data,
        destination: to,
        nonce,
        value,
    })
}

/// Encode a payload from the function selection and calldata.
fn create_payload(
    target: ContractId,
    function_selector: Bytes,
    calldata: Bytes,
    single_value_type_arg: bool,
) -> Bytes {
    /*
    packs args according to spec (https://github.com/FuelLabs/fuel-specs/blob/master/src/vm/instruction_set.md#call-call-contract) :
    bytes   type        value   description
    32	    byte[32]    to      Contract ID to call.
    8	    byte[8]	    param1  First parameter (function selector).
    8	    byte[8]	    param2  Second parameter (abi-encoded calldata: value if value type, otherwise pointer to reference type).
    */
    require(function_selector.len() == 8, "function selector must be 8 bytes");

    let mut payload = Bytes::new().join(contract_id_to_bytes(target)).join(function_selector);

    if (single_value_type_arg) {
        payload = payload.join(calldata); // When calldata is copy type, just pass calldata
    } else {
        payload = payload.join(ptr_as_bytes(calldata.buf.ptr)); // When calldata is reference type, need to get pointer as bytes
    };

    payload
}

// TODO : Replace with `from` when implemented
/// Represent an address as a `Bytes`.
fn address_to_bytes(address: Address) -> Bytes {
    let mut target_bytes = Bytes::with_capacity(32);
    target_bytes.len = 32;

    __addr_of(address).copy_bytes_to(target_bytes.buf.ptr, 32);

    target_bytes
}

// TODO : Replace with `from` when implemented
/// Represent a contract ID as a `Bytes`, so it can be concatenated with a payload.
fn contract_id_to_bytes(contract_id: ContractId) -> Bytes {
    let mut target_bytes = Bytes::with_capacity(32);
    target_bytes.len = 32;

    __addr_of(contract_id).copy_bytes_to(target_bytes.buf.ptr, 32);

    target_bytes
}

/// Represent a raw pointer as a `Bytes`, so it can be concatenated with a payload.
fn ptr_as_bytes(ptr: raw_ptr) -> Bytes {

    let mut bytes = Bytes::with_capacity(8);
    bytes.len = 8;

    // Need to copy pointer to heap so it has an address and can be copied onto the bytes buffer
    let mut ptr_on_heap = Vec::new();
    ptr_on_heap.push(ptr);
    ptr_on_heap.buf.ptr.copy_bytes_to(bytes.buf.ptr, 8);

    bytes
}

/// Applies the format and prefix specified by signature_info to the message_hash.
/// Returns the b256 value of the recovered address.
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

/// EIP-191: https://eips.ethereum.org/EIPS/eip-191
/// Creates an EIP-191 compliant transaction hash, of the version:
/// 0x45 - personal sign.
/// It takes a `data_to_sign` to represent the <data to sign> in the EIP-191 format:
/// 0x19 <1 byte version> <version specific data> <data to sign>
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

/// Encode the `initial_byte`, `version_byte` and `message_hash` into a Vec<u64> of length 40 bytes,
/// where the first 34 bytes are the desired `signed_data` tightly packed.
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

/// Get a tuple of 4 u64 values from a single b256 value.
fn decompose(val: b256) -> (u64, u64, u64, u64) {
    asm(r1: __addr_of(val)) { r1: (u64, u64, u64, u64) }
}

/// Applies the prefix "\x19Ethereum Signed Message:\n32" to a message hash.
fn ethereum_prefix(msg_hash: b256) -> b256 {
    keccak256((ETHEREUM_PREFIX, msg_hash))
}
