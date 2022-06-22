library data_structures;

use std::{address::Address, contract_id::ContractId, identity::Identity, vec::Vec};

pub struct User {
    // Contracts cannot sign therefore restrict scope to Address
    /// The wallet address of a user
    identity: Address,

    /// Number of approvals the user provides when approving.
    /// The default is usually 1
    weight: u64,
}

pub struct Transaction {
    /// Unique identifier for the contract which prevents this Tx from being submitted to another
    /// instance of the multisig
    contract_identifier: ContractId,

    /// Payload sent to destination
    data: Vec<u64>,

    /// The recipient (output / contract) regarding the Tx details
    destination: Identity,

    /// Value used to prevent double spending
    nonce: u64,

    /// Amount of asset
    value: u64,
}
