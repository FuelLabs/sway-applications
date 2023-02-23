library interface;

dep data_structures;

use data_structures::ClaimData;

abi AirdropDistributor {
    /// This function will let users claim their airdrop.
    ///
    /// A Merkle proof will need to be provided to claim the airdrop. This is then verified against
    /// the merkle root and a hash of the (`Identity`, `u64`) tuple as the leaf.
    /// This function uses the Binary Merkle Proof library in Sway-Libs and inherits it's specs.
    ///
    /// # Arguments
    ///
    /// * `amount` - The quantity of an asset allotted to the user to claim.
    /// * 'key' - The index of the leaf which will be proven on the Merkle Tree.
    /// * `num_leaves` - The number of leaves in the Merkle Tree.
    /// * `proof` - The Merkle proof to verify the user is authorized to claim.
    /// * `to` - The user which has been allotted a quantity of the asset.
    ///
    /// # Reverts
    ///
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]
    fn claim(amount: u64, key: u64, num_leaves: u64, proof: Vec<b256>, to: Identity);

    /// Initialized the contract and starts the airdrop.
    ///
    /// Note: The `asset` contract will need to have a `mint_to` function implemented which this
    /// airdrop contract may call.
    ///
    /// # Arguments
    ///
    /// * `asset` - The contract which is to be distributed.
    /// * `claim_time` - The number fo blocks the claiming period should last.
    /// * `merkleRoot` - The root of the merkle proof used to verify claiming.
    ///
    /// # Reverts
    ///
    /// * The constructor has already been called.
    #[storage(read, write)]
    fn constructor(asset: ContractId, claim_time: u64, merkleRoot: b256);
}

abi Info {
    /// Returns the claim data stored on the given identity
    ///
    /// # Arguments
    ///
    /// * `identity` - The user whose ClaimData will be returned
    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimData;

    /// Returns the block at which the airdrop ends
    #[storage(read)]
    fn end_block() -> u64;

    /// Returns the merkle root of the airdrop used to verify proofs
    ///
    /// # Reverts
    ///
    /// * When the contract has not yet been initalized
    #[storage(read)]
    fn merkle_root() -> b256;
}

abi SimpleAsset {
    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity);
}
