library hashing;

dep user;

use std::{b512::B512, bytes::Bytes, constants::ZERO_B256};
use user::User;

impl Bytes {
    pub fn from_copy_type<T>(value: T) -> Bytes {
    // Artificially create bytes with capacity and len
        let mut bytes = Bytes::with_capacity(8);
        bytes.len = 8;

        asm(buffer, ptr: value, dst: bytes.buf.ptr, len: 8) {
            move buffer sp; // Make `buffer` point to the current top of the stack
            cfei i8; // Grow stack by 1 word
            sw   buffer ptr i0; // Save value in register at `ptr` to memory at `buffer`
            mcp  dst buffer len; // Copy `len` bytes in memory starting from `buffer`, to `dst`
            cfsi i8; // Shrink stack by 1 word
        }

        bytes
    }

    pub fn from_reference_type<T>(t: T) -> Bytes { // NOTE: Does not work correctly for Option<Bytes>, use `from_option_bytes` instead
        // Artificially create bytes with capacity and len
        let size = __size_of::<T>();
        let mut bytes = Bytes::with_capacity(size);
        bytes.len = size;
        // Copy bytes from contract_id into the buffer of the target bytes
        __addr_of(t).copy_bytes_to(bytes.buf.ptr, size);
        bytes
    }
}

impl Bytes {
    pub fn from_option_bytes(o: Option<Bytes>) -> Bytes {
        match o {
            Option::None => { 
            // __size_of_val(o) == 32 bytes
            // 8 bytes for the enum tag (0u64) + 24 bytes for the Bytes type ([0u8; 24])
                Bytes::from_reference_type(ZERO_B256)
            },
            Option::Some(bytes) => {
                let mut option_bytes = Bytes::from_copy_type(1u64);
                option_bytes.append(bytes);
                option_bytes
            }
        }
    }
}

pub trait IntoBytes {
    fn into_bytes(self) -> Bytes;
}

pub struct Transaction {
    asset_id: Option<ContractId>,
    calldata: Option<Bytes>,
    contract_identifier: ContractId,
    forwarded_gas: Option<u64>,
    function_selector: Option<Bytes>,
    nonce: u64,
    single_value_type_arg: Option<bool>,
    target: Identity,
    value: Option<u64>,
}

impl IntoBytes for Transaction { // Needed as `Transaction` contains `Bytes` which can only be correctly hashed by the Bytes.sha256() method, 
                                 // as such the whole struct must be converted to `Bytes`.
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.append(Bytes::from_reference_type(self.asset_id));
        bytes.append(Bytes::from_option_bytes(self.calldata));
        bytes.append(Bytes::from_reference_type(self.contract_identifier));
        bytes.append(Bytes::from_reference_type(self.forwarded_gas));
        bytes.append(Bytes::from_option_bytes(self.function_selector));
        bytes.append(Bytes::from_copy_type(self.nonce));
        bytes.append(Bytes::from_reference_type(self.single_value_type_arg));
        bytes.append(Bytes::from_reference_type(self.target));
        bytes.append(Bytes::from_reference_type(self.value));
        bytes
    }
}

pub struct Threshold {
    contract_identifier: ContractId,
    nonce: u64,
    threshold: u64,
}

pub enum TypeToHash {
    //Transaction: Transaction, // TODO: Uncomment when the RustSDK supports `Bytes`. https://github.com/FuelLabs/fuels-rs/issues/723.
    Threshold: Threshold,
}
