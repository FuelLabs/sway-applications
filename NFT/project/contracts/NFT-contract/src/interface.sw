library interface;

dep data_structures;

use data_structures::TokenMetadata;

abi Auxiliary {
    /// Sets the inital state and unlocks the functionality for the rest of the contract.
    ///
    /// This function can only be called once.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The administrator to be set for this contract.
    /// * `new_max_supply` - The maximum supply of tokens that may ever be minted.
    ///
    /// # Reverts
    ///
    /// * When the contract has already been initalized
    #[storage(read, write)]
    fn constructor(new_admin: Option<Identity>, new_max_supply: Option<u64>);

    /// Returns the metadata for a specific token.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The id of the token which the metadata should be returned.
    #[storage(read)]
    fn token_metadata(token_id: u64) -> Option<TokenMetadata>;
}
