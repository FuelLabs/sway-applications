library;

use ::data_structures::user::User;
use std::{bytes::Bytes, constants::ZERO_B256, hash::{Hash, Hasher}};

impl Bytes {
    /// Converts a generic copy type into [Bytes].
    pub fn from_copy_type<T>(value: T) -> Self {
        // Artificially create bytes with capacity and len
        let mut bytes = Bytes::with_capacity(8);
        bytes.len = 8;

        asm(buffer, ptr: value, dst: bytes.buf.ptr, len: 8) {
            move buffer sp;
            cfei i8;
            sw buffer ptr i0;
            mcp dst buffer len;
            cfsi i8;
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
    /// Parameters for a transfer.
    transfer_params: TransferParams,
}

impl IntoBytes for ContractCallParams {
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.append(self.calldata);
        bytes.append(Bytes::from_copy_type(self.forwarded_gas));
        bytes.append(self.function_selector);
        bytes.append(Bytes::from_copy_type(self.single_value_type_arg));
        bytes.append(Bytes::from_reference_type(self.transfer_params));
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

impl Threshold {
    pub fn new(contract_identifier: ContractId, nonce: u64, threshold: u64) -> Self {
        Self {
            contract_identifier,
            nonce,
            threshold,
        }
    }
}

/// Determines the type of transaction parameters.
pub enum TransactionParameters {
    Call: ContractCallParams,
    Transfer: TransferParams,
}

impl IntoBytes for TransactionParameters {
    fn into_bytes(self) -> Bytes {
        match self {
            TransactionParameters::Call(contract_call_params) => {
                // As [ContractCallParams] contains fields of type [Bytes], manual serialisation is necessary.
                let mut bytes = Bytes::from_copy_type(0u64);
                bytes.append(contract_call_params.into_bytes());
                bytes
            },
            TransactionParameters::Transfer => {
                Bytes::from_reference_type(self)
            },
        }
    }
}

/// The data to be hashed and signed over when calling `execute_transaction`.
pub struct Transaction {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    nonce: u64,
    /// The target of the transaction.
    target: Identity,
    /// Parameters of the transaction.
    transaction_parameters: TransactionParameters,
}

impl Transaction {
    pub fn new(
        contract_identifier: ContractId,
        nonce: u64,
        target: Identity,
        transaction_parameters: TransactionParameters,
    ) -> Self {
        Self {
            contract_identifier,
            nonce,
            target,
            transaction_parameters,
        }
    }
}

impl IntoBytes for Transaction {
    // Needed as [Transaction] contains [TransactionParameters], which itself may contain [Bytes] which can only be correctly hashed by the Bytes.sha256() method, 
    // as such the whole struct must be serialised to [Bytes].
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.append(Bytes::from_reference_type(self.contract_identifier));
        bytes.append(Bytes::from_copy_type(self.nonce));
        bytes.append(Bytes::from_reference_type(self.target));
        bytes.append(self.transaction_parameters.into_bytes());
        bytes
    }
}

/// Parameters for a transfer.
pub struct TransferParams {
    /// The asset to transfer.
    asset_id: AssetId,
    /// The amount to transfer.
    value: Option<u64>,
}

/// Determines the type to be hashed.
pub enum TypeToHash {
    Threshold: Threshold,
    Transaction: Transaction,
    Weight: Weight,
}

impl Hash for User {
    fn hash(self, ref mut state: Hasher) {
        self.address.hash(state);
        self.weight.hash(state);
    }
}

impl Hash for Threshold {
    fn hash(self, ref mut state: Hasher) {
        self.contract_identifier.hash(state);
        self.nonce.hash(state);
        self.threshold.hash(state);
    }
}

impl Hash for Transaction {
    fn hash(self, ref mut state: Hasher) {
        self.contract_identifier.hash(state);
        self.nonce.hash(state);
        self.target.hash(state);
        self.transaction_parameters.into_bytes().hash(state);
    }
}

impl Hash for Weight {
    fn hash(self, ref mut state: Hasher) {
        self.contract_identifier.hash(state);
        self.nonce.hash(state);
        self.user.hash(state);
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

impl Weight {
    pub fn new(contract_identifier: ContractId, nonce: u64, user: User) -> Self {
        Self {
            contract_identifier,
            nonce,
            user,
        }
    }
}
