library;

use ::data_structures::user::User;

pub struct Transaction {
    /// Unique identifier for the contract which prevents this transaction from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// Payload sent to destination  // TODO: change to Bytes when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723
    data: b256,
    /// The recipient (output / contract) regarding the transaction details.
    destination: Identity,
    /// Value used to prevent double spending.
    nonce: u64,
    /// Amount of asset.
    value: u64,
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
