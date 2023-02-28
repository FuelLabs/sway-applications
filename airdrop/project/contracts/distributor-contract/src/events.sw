library events;

pub struct ClaimEvent {
    /// The quantity of an asset which is to be transfered to the user.
    amount: u64,
    /// The user that has a claim to tokens with a valid proof.
    claimer: Identity,
    /// The identity that will recieve the transfered asset.
    to: Identity,
}

pub struct ClawbackEvent {
    /// The quantity of an asset which will be returned after the claiming period has ended.
    amount: u64,
    /// The user that will recieve the remaining asset balance.
    to: Identity,
}

pub struct CreateAirdropEvent {
    /// The user which can claim any left over tokens after the claiming period.
    admin: Identity,
    /// The asset which is to be distributed.
    asset: ContractId,
    /// The block at which the claiming period will end.
    end_block: u64,
    /// The computed merkle root that will be used to verify claims.
    merkle_root: b256,
    /// The total number of leaves in the merkle tree
    num_leaves: u64,
}
