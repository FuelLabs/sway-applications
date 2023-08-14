library;

use ::data_structures::ClaimState;

abi AirdropDistributor {
    /// This function will let users claim their airdrop.
    ///
    /// # Additional Information
    ///
    /// A Merkle proof will need to be provided to claim the airdrop. This is then verified against
    /// the merkle root and a hash of the (`Identity`, `u64`) tuple as the leaf.
    /// This function uses the Binary Merkle Proof library in Sway-Libs and inherits it's specs.
    ///
    /// # Arguments
    ///
    /// * `amount`: [u64] - The quantity of tokens allotted to the user to claim.
    /// * `key`: [u64] - The index of the leaf which will be proven on the Merkle Tree.
    /// * `proof`: [Vec<b256>] - The Merkle proof to verify the user is authorized to claim.
    /// * `to`: [Identity] - The user which has been allotted a quantity of the tokens.
    ///
    /// # Reverts
    ///
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the contract's balance is less than the `amount` to claim.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]
    fn claim(amount: u64, key: u64, proof: Vec<b256>, to: Identity);

    /// Returns any unclaimed tokens to 'admin' when the claiming period of the airdrop has ended.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the contract's admin.
    /// * When the claiming period is still active.
    /// * When there are no tokens to claim.
    #[storage(read)]
    fn clawback();

    /// Initializes the contract and starts the airdrop.
    ///
    /// # Arguments
    ///
    /// * `admin`: [Identity] - The user which has the ability to clawback any unclaimed tokens.
    /// * `claim_time`: [u64] - The number fo blocks the claiming period should last.
    /// * `merkleRoot`: [b256] - The root of the merkle proof used to verify claiming.
    /// * `num_leaves`: [u64] - The number of leaves in the Merkle Tree.
    ///
    /// # Reverts
    ///
    /// * When the constructor has already been called.
    /// * When no tokens are sent to the airdrop contract.
    #[payable]
    #[storage(read, write)]
    fn constructor(admin: Identity, claim_time: u64, merkle_root: b256, num_leaves: u64);
}

abi Info {
    /// Returns the user which has the ability to clawback any unclaimed tokens.
    ///
    /// # Returns
    ///
    /// * [Option<Identity>] - The user which has the ability to clawback any unclaimed tokens.
    #[storage(read)]
    fn admin() -> Option<Identity>;

    /// Returns the claim data stored on the given identity.
    ///
    /// # Arguments
    ///
    /// * `identity`: [Identity] - The user whose ClaimState will be returned.
    ///
    /// # Returns
    ///
    /// * [ClaimState] - The ClaimState of the given identity.
    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimState;

    /// Returns the block at which the airdrop ends
    ///
    /// # Returns
    ///
    /// * [u64] - The block at which the airdrop ends.
    #[storage(read)]
    fn end_block() -> u64;

    /// Returns whether the airdrop is active and tokens can be claimed.
    ///
    /// # Returns
    ///
    /// * [bool] - Whether the airdrop is active and tokens can be claimed.
    #[storage(read)]
    fn is_active() -> bool;

    /// Returns the merkle root of the airdrop used to verify proofs.
    ///
    /// # Returns
    ///
    /// * [Option<b256>] - The merkle root of the airdrop used to verify proofs.
    #[storage(read)]
    fn merkle_root() -> Option<b256>;

    /// Returns the number of leaves within the merkle tree.
    ///
    /// # Returns
    ///
    /// * [u64] - The number of leaves within the merkle tree.
    #[storage(read)]
    fn number_of_leaves() -> u64;
}

abi SimpleAsset {
    /// Mints the given amount of tokens to the given Identity.
    ///
    /// # Arguments
    ///
    /// * `amount`: [u64] - The quantity of tokens to mint.
    /// * `to`: [Identity] - The user which will receive the minted tokens.
    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity);
}
