library events;

use std::{identity::Identity, option::Option, vec::Vec};

pub struct ApprovalEvent {
    /// An `Option` of the `Identity` that has gotten approval. If an approval was revoked, the
    /// `Option` will be `None`.
    approved: Option<Identity>,

    /// The `Identity` of the token's owner that has given or revoked approval.
    owner: Identity,

    /// The `u64` id of the token which has given or revoked approval.
    token_id: u64,
}

pub struct BurnEvent {
    /// The `Identity` of the token's owner that has burned the token.
    owner: Identity,

    /// The `u64` id of the token which has been burned.
    token_id: u64,
}

pub struct MintEvent {
    /// The `Identity` of the newly minted tokens which has been set as the owner.
    owner: Identity,

    /// The `Vec` of tokens which have been minted in this transaction.
    token_ids: Vec<u64>,
}

pub struct OperatorEvent {
    /// The `bool` which determines whether approval has been given or revoked to be an operator.
    approve: bool,
    
    /// The `Identity` which has been given or revoked approval to be an operator to the owner.
    operator: Identity,

    /// The `Identity` which has given or revoked approval to the operator.
    owner: Identity,
}

pub struct TransferEvent {
    /// The `Identity` which previously owned the token.
    from: Identity,

    // The `Identity` that made the transfer, this can be the owner, the approved `Identity`, or the operator.
    sender: Identity,

    /// The `Identity` which now owns the token.
    to: Identity,

    /// The `u64` id of the token which has transfered ownership.
    token_id: u64,
}
