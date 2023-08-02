library;

use ::data_structures::user::User;
use std::{bytes::Bytes, constants::ZERO_B256};

impl Bytes {
    /// Converts a generic copy type into [Bytes].
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

    /// Converts a generic reference type into [Bytes].
    pub fn from_reference_type<T>(t: T) -> Self {
        // Artificially create bytes with capacity and len
        let size = __size_of::<T>();
        let mut bytes = Bytes::with_capacity(size);
        bytes.len = size;
        // Copy bytes of `t` into the buffer of the target bytes
        __addr_of(t).copy_bytes_to(bytes.buf.ptr, size);
        bytes
    }
}

pub trait IntoBytes {
    /// Converts self into [Bytes].
    fn into_bytes(self) -> Bytes;
}

/// Parameters for calling a contract.
pub struct ContractCallParams {
    /// The calldata for the call.
    calldata: Bytes,
    /// The amount of gas to forward.
    forwarded_gas: u64,
    /// The function selector for the call.
    function_selector: Bytes,
    /// Whether the function being called takes a single value-type argument.
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

/// The data to be hashed and signed over when calling `set_threshold`.
pub struct Threshold {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    nonce: u64,
    /// The number of approvals required to enable a transaction to be sent.
    threshold: u64,
}

/// Parameters for a transfer.
pub struct TransferParams {
    /// The asset to transfer.
    asset_id: ContractId,
    /// The amount to transfer.
    value: Option<u64>,
}

/// The data to be hashed and signed over when calling `execute_transaction`.
pub struct Transaction {
    /// Parameters for calling a contract.
    contract_call_params: Option<ContractCallParams>,
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    nonce: u64,
    /// The target of the transaction.
    target: Identity,
    /// Parameters for a transfer.
    transfer_params: TransferParams,
}

impl IntoBytes for Transaction {
    // Needed as [Transaction] contains [Option<ContractCallParams>], which itself contains [Bytes] which can only be correctly hashed by the Bytes.sha256() method, 
    // as such the whole struct must be converted to [Bytes].
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        match self.contract_call_params {
            Option::None => {
                bytes.append(Bytes::from_reference_type(self.contract_call_params));
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

/// The data to be hashed and signed over when calling `set_weight`.
pub struct Weight {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    nonce: u64,
    /// The user of the multisig, who can sign transactions to add their approval.
    user: User,
}

/// Determines the type to be hashed.
pub enum TypeToHash {
    Threshold: Threshold,
    Transaction: Transaction,
    Weight: Weight,
}
