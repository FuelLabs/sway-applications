library interface;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

abi AirdropDistributor {
    /// This function will let users claim their airdrop.
    ///
    /// A Merkle proof will need to be provided to claim the airdrop. This is then verified against
    /// the merkle root and a hash of the (`to`, `amount`) tuple.
    ///
    /// # Arguments
    ///
    /// * `amount` - The quantity of tokens the allotted to the user to claim.
    /// * `proof` - The Merkle proof to verify the user is authorized to claim.
    /// * `to` - The user which has been allotted tokens.
    ///
    /// # Reverts
    ///
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]fn claim(amount: u64, proof: Vec<b256>, to: Identity);

    /// Initialized the contract and starts the airdrop.
    ///
    /// Note: The `token` contract will need to have a `mint_to` function implemented which this
    /// airdrop contract may call.
    ///
    /// # Arguments
    ///
    /// * `claim_time` - The number fo blocks the claiming period should last.
    /// * `merkleRoot` - The root of the merkle proof used to verify claiming.
    /// * `token` - The contract which is to be distributed.
    ///
    /// # Reverts
    ///
    /// * The constructor has already been called.
    /// * The `claim_time` is set to zero.
    #[storage(read, write)]fn constructor(claim_time: u64, merkleRoot: b256, token: ContractId);
}

abi SimpleToken {
    fn mint_to(amount: u64, to: Identity);
}
