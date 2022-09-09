library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ClaimEvent {
    /// The quantity of an asset which is to be minted to the user.
    amount: u64,
    /// The user that will be recieving the minted asset.
    to: Identity,
}

pub struct CreateAirdropEvent {
    /// The asset which is to be distributed and has an implemented `mint_to` function.
    asset: ContractId,
    /// The block at which the minting period will end.
    end_block: u64,
    /// The computed merkle root that will be used to verify claims.
    merkle_root: b256,
}
