library data_structures;

use std::{address::Address, contract_id::ContractId, identity::Identity};

pub struct Owner {
    /// Has a specific address been marked as an owner in the contract
    exists: bool,

    /// Number of approvals the user provides when approving.
    /// The default is usually 1
    weight: u64,
}

pub struct Transaction {
    /// Unique identifier for the contract which prevents this Tx from being submitted to another
    /// instance of the multisig
    contract_identifier: ContractId,

    /// Payload sent to destination
    data: [u64; 3],

    /// The recipient (output / contract) regarding the Tx details
    destination: Identity,

    /// Value used to prevent double spending
    nonce: u64,

    /// Amount of asset
    value: u64,
}

pub struct User {
    // Contracts cannot sign therefore restrict scope to Address
    /// The wallet address of a user
    identity: Address,

    /// Number of approvals the user provides when approving.
    /// The default is usually 1
    weight: u64,
}
