library;

use ::data_structures::user::User;
use std::{bytes::Bytes, constants::ZERO_B256};

impl Bytes {
    pub fn from_copy_type<T>(value: T) -> Self {
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

    pub fn from_reference_type<T>(t: T) -> Self { // NOTE: Does not work correctly for Option<Bytes>, use `from_option_bytes` instead
        // Artificially create bytes with capacity and len
        let size = __size_of::<T>();
        let mut bytes = Bytes::with_capacity(size);
        bytes.len = size;
        // Copy bytes into the buffer of the target bytes
        __addr_of(t).copy_bytes_to(bytes.buf.ptr, size);
        bytes
    }
}

pub trait IntoBytes {
    fn into_bytes(self) -> Bytes;
}

pub struct ContractCallParams {
    calldata: Bytes,
    forwarded_gas: u64,
    function_selector: Bytes,
    single_value_type_arg: bool,
}

impl IntoBytes for ContractCallParams {
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.append(self.calldata);
        bytes.append(Bytes::from_copy_type(self.forwarded_gas));
        bytes.append(self.function_selector);
        bytes.append(Bytes::from_copy_type(self.single_value_type_arg));
        bytes
    }
}

pub struct Threshold {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// Payload sent to destination  // TODO: change to Bytes when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723
    data: Option<b256>,
    /// Value used to prevent double spending.
    nonce: u64,
    /// The number of approvals required to enable a transaction to be sent.
    threshold: u64,
}

pub struct TransferParams {
    asset_id: ContractId,
    value: Option<u64>,
}

pub struct Transaction {
    contract_call_params: Option<ContractCallParams>,
    contract_identifier: ContractId,
    nonce: u64,
    target: Identity,
    transfer_params: TransferParams,
}

impl IntoBytes for Transaction {
    // Needed as `Transaction` contains `Bytes` which can only be correctly hashed by the Bytes.sha256() method, 
    // as such the whole struct must be converted to `Bytes`.
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        match self.contract_call_params {
            Option::None => {
                // __size_of_val(self.contract_call_params) == 32 bytes
                bytes.append(Bytes::from_reference_type(ZERO_B256))
            },
            Option::Some(contract_call_params) => {
                let mut serialised_option = Bytes::from_copy_type(1u64);
                serialised_option.append(contract_call_params.into_bytes());
                bytes.append(serialised_option)
            }
        }
        bytes.append(Bytes::from_reference_type(self.contract_identifier));
        bytes.append(Bytes::from_copy_type(self.nonce));
        bytes.append(Bytes::from_reference_type(self.target));
        bytes.append(Bytes::from_reference_type(self.transfer_params));
        bytes
    }
}

pub struct Weight {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// Payload sent to destination  // TODO: change to Bytes when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723
    data: Option<b256>,
    /// Value used to prevent double spending.
    nonce: u64,
    /// The user of the multisig, who can sign transactions to add their approval.
    user: User,
}
