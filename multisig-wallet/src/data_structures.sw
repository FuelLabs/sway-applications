library data_structures;

use std::{address::Address, contract_id::ContractId, identity::Identity};

pub struct User {
    // Contracts cannot sign therefore restrict scope to Address
    /// The wallet address of a user
    identity: Address,

    /// Number of approvals the user provides when approving.
    /// The default is usually 1
    weight: u64,
}

pub struct Transaction {
    /// The recipient (output / contract) regarding the Tx details
    destination: Identity,

    /// Amount of asset
    value: u64,

    /// Payload sent to destination  // TODO: change to vec when implemented
    data: b256,

    /// Value used to prevent double spending
    nonce: u64,

    /// Unique identifier for the contract which prevents this Tx from being submitted to another
    /// instance of the multisig
    contract_identifier: ContractId,
}
