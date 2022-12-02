library interface;

dep data_structures;

use data_structures::TokenMetadata;

abi Custom {
    /// Sets the inital state and unlocks the functionality for the rest of the contract.
    ///
    /// This function can only be called once.
    ///
    /// # Arguments
    ///
    /// * `admin` - The only user which has the ability to mint.
    /// * `max_supply` - The maximum supply of tokens that can ever be minted.
    ///
    /// # Reverts
    ///
    /// * When the contract has already been initalized
    #[storage(read, write)]
    fn constructor(new_admin: Option<Identity>, new_max_supply: Option<u64>);

    #[storage(read)]
    fn token_metadata(token_id: u64) -> Option<TokenMetadata>;
}
