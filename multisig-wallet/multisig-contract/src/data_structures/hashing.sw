library;

use ::data_structures::user::User;
use std::{alloc::alloc, bytes::Bytes, constants::ZERO_B256, hash::{Hash, Hasher}};

impl Bytes {
    /// Converts a generic type into [Bytes].
    pub fn from_type<T>(value: T) -> Self {
        let ptr = alloc::<T>(1);
        ptr.write(value);

        let slice = raw_slice::from_parts::<T>(ptr, 1);

        Bytes::from(slice)
    }
}

pub trait IntoBytes {
    /// Converts self into [Bytes].
    fn into_bytes(self) -> Bytes;
}

/// Parameters for calling a contract.
pub struct ContractCallParams {
    /// The calldata for the call.
    pub calldata: Bytes,
    /// The amount of gas to forward.
    pub forwarded_gas: u64,
    /// The function selector for the call.
    pub function_selector: Bytes,
    /// Whether the function being called takes a single value-type argument.
    pub single_value_type_arg: bool,
    /// Parameters for a transfer.
    pub transfer_params: TransferParams,
}

impl IntoBytes for ContractCallParams {
    fn into_bytes(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.append(self.calldata);
        bytes.append(Bytes::from_type(self.forwarded_gas));
        bytes.append(self.function_selector);
        bytes.append(Bytes::from_type(self.single_value_type_arg));
        bytes.append(Bytes::from_type(self.transfer_params));
        bytes
    }
}

/// The data to be hashed and signed over when calling `set_threshold`.
pub struct Threshold {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    pub contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    pub nonce: u64,
    /// The number of approvals required to enable a transaction to be sent.
    pub threshold: u64,
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
                let mut bytes = Bytes::from_type(0u64);
                bytes.append(contract_call_params.into_bytes());
                bytes
            },
            TransactionParameters::Transfer(transfer_params) => {
                Bytes::from_type(transfer_params)
            },
        }
    }
}

/// The data to be hashed and signed over when calling `execute_transaction`.
pub struct Transaction {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    pub contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    pub nonce: u64,
    /// The target of the transaction.
    pub target: Identity,
    /// Parameters of the transaction.
    pub transaction_parameters: TransactionParameters,
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
        bytes.append(Bytes::from_type(self.contract_identifier));
        bytes.append(Bytes::from_type(self.nonce));
        bytes.append(Bytes::from_type(self.target));
        bytes.append(self.transaction_parameters.into_bytes());
        bytes
    }
}

/// Parameters for a transfer.
pub struct TransferParams {
    /// The asset to transfer.
    pub asset_id: AssetId,
    /// The amount to transfer.
    pub value: Option<u64>,
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
    pub contract_identifier: ContractId,
    /// The nonce of the multisig wallet, used to prevent double spending.
    pub nonce: u64,
    /// The user of the multisig, who can sign transactions to add their approval.
    pub user: User,
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

// #[test]
// fn test_convert_u64_to_bytes() {
//     use std::bytes_conversions::u64::*;
//     use std::bytes::*;
//     let u64_1 = 1u64;
//     let result_bytes = Bytes::from_type(u64_1);

//     let expected_bytes = Bytes::from(u64_1.to_be_bytes());
//     assert_eq(result_bytes, expected_bytes);
// }

// #[test]
// fn test_convert_bool_to_bytes() {
//     let bool_1 = true;

//     let result_bytes = Bytes::from_type(bool_1);

//     let mut expected_bytes = Bytes::new();
//     expected_bytes.push(1_u8);

//     assert_eq(result_bytes, expected_bytes);
// }

// #[test]
// fn test_convert_transfer_params_to_bytes() {
//     use std::bytes_conversions::u64::*;
//     use std::bytes::*;
//     let transfer_params = TransferParams {
//         asset_id: AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000001),
//         value: Some(100),
//     };

//     let result_bytes = Bytes::from_type(transfer_params);

//     let mut expected_bytes = Bytes::new();
//     expected_bytes.append(Bytes::from(transfer_params.asset_id.bits()));
//     expected_bytes.append(Bytes::from(1_u64.to_be_bytes()));
//     expected_bytes.append(Bytes::from(100_u64.to_be_bytes()));

//     assert_eq(result_bytes, expected_bytes);
// }
