library data_structures;

use std::{b512::B512, contract_id::ContractId, identity::Identity};

pub struct User {
    /// The wallet address of a user.
    address: b256,
    /// Number of approvals the user provides when approving.
    /// The default is usually 1.
    weight: u64,
}

pub struct Transaction {
    /// Unique identifier for the contract which prevents this Tx from being submitted to another
    /// instance of the multisig.
    contract_identifier: ContractId,
    /// Payload sent to destination  // TODO: change to Bytes when implemented.
    data: b256,
    /// The recipient (output / contract) regarding the Tx details.
    destination: Identity,
    /// Value used to prevent double spending.
    nonce: u64,
    /// Amount of asset.
    value: u64,
}

pub enum MessageFormat {
    None: (),
    EIP191PersonalSign: (),
}

pub enum MessagePrefix {
    None: (),
    Ethereum: (),
}

pub enum WalletType {
    Fuel: (),
    EVM: (),
}

pub struct SignatureData {
    signature: B512,
    /// The type of formmatting of the message that was signed.
    format: MessageFormat,
    /// The type of prefix prepended to the message that was signed.
    prefix: MessagePrefix,
    /// The wallet type of the signer of the message.
    wallet_type: WalletType,
}
