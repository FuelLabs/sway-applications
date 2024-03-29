library;

pub struct ClaimEvent {
    /// The quantity of an asset which is to be transferred to the user.
    amount: u64,
    /// The user that has a claim to coins with a valid proof.
    claimer: Identity,
    /// The identity that will receive the transferred asset.
    to: Identity,
}

pub struct ClawbackEvent {
    /// The quantity of an asset which will be returned after the claiming period has ended.
    amount: u64,
    /// The user that will receive the remaining asset balance.
    to: Identity,
}

pub struct CreateAirdropEvent {
    /// The user which can claim any left over coins after the claiming period.
    admin: Identity,
    /// The asset which is to be distributed.
    asset: AssetId,
    /// The block at which the claiming period will end.
    end_block: u32,
    /// The computed merkle root that will be used to verify claims.
    merkle_root: b256,
    /// The total number of leaves in the merkle tree
    number_of_leaves: u64,
}
