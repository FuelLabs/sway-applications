library events;

use std::{
    contract_id::ContractId,
    identity::Identity,
};

pub struct ClaimEvent {
    /// The quantity of tokens which is to be minted to the user.
    amount: u64,
    /// The user that will be recieving the minted tokens.
    to: Identity,
}

pub struct InitializeEvent {
    /// The block at which the minting period will end.
    end_block: u64,
    /// The computed merkle root that will be used to verify claims.
    merkle_root: b256,
    /// The token which is to be distributed and has an implemented `mint_to` function.
    token_contract: ContractId,
}
