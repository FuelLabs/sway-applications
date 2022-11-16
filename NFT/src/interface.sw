library interface;

dep data_structures;

use data_structures::NFTMetadata;

abi NFT {
    /// Returns the current admin for the contract.
    #[storage(read)]
    fn admin() -> Option<Identity>;

    /// Gives approval to the `approved` user to transfer a specific token on another user's behalf.
    ///
    /// To revoke approval the approved user should be `None`.
    ///
    /// # Arguments
    ///
    /// * `approved_identity` - The user which will be allowed to transfer the token on the owner's behalf.
    /// * `token_id` - The unique identifier of the token which the owner is giving approval for.
    #[storage(read, write)]
    fn approve(approved_identity: Option<Identity>, token_id: u64);

    /// Returns the user which is approved to transfer the given token.
    ///
    /// If there is no approved user or the unique identifier does not map to an existing token,
    /// the function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token which the approved user should be returned.
    #[storage(read)]
    fn approved(token_id: u64) -> Option<Identity>;

    /// Returns the balance of the `owner` user.
    ///
    /// # Arguments
    ///
    /// * `owner` - The user of which the balance should be returned.
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;

    /// Burns the specified token.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token which is to be burned.
    #[storage(read, write)]
    fn burn(token_id: u64);

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

    /// Returns whether the `operator` user is approved to transfer all tokens on the `owner`
    /// user's behalf.
    ///
    /// # Arguments
    ///
    /// * `operator` - The user which has recieved approval to transfer all tokens on the `owner`s behalf.
    /// * `owner` - The user which has given approval to transfer all tokens to the `operator`.
    #[storage(read)]
    fn is_approved_for_all(operator: Identity, owner: Identity) -> bool;

    /// Returns the total number of tokens which will ever be minted.
    #[storage(read)]
    fn max_supply() -> Option<u64>;

    /// Mints `amount` number of tokens to the `to` `Identity`.
    ///
    /// Once a token has been minted, it can be transfered and burned.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of tokens to be minted in this transaction.
    /// * `to` - The user which will own the minted tokens.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initalized
    /// * When an admin exsits and the sender is not the admin
    /// * When a max supply exists and the sender is not the admin
    #[storage(read, write)]
    fn mint(amount: u64, to: Identity);

    /// Returns the metadata for the token specified
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token.
    #[storage(read)]
    fn meta_data(token_id: u64) -> Option<NFTMetadata>;

    /// Returns the user which owns the specified token.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token.
    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity>;

    /// Changes the contract's admin.
    ///
    /// This new admin will have access to minting if `access_control` is set to true and be able
    /// to change the contract's admin to a new admin.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The user which is to be set as the new admin.
    ///
    /// # Reverts
    ///
    /// * When there is no contract admin.
    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>);

    /// Gives the `operator` user approval to transfer ALL tokens owned by the `owner` user.
    ///
    /// This can be dangerous. If a malicous user is set as an operator to another user, they could
    /// drain their wallet.
    ///
    /// # Arguments
    ///
    /// * `approval` - Represents whether the user is giving or revoking operator status.
    /// * `operator` - The user which may transfer all tokens on the owner's behalf.
    #[storage(write)]
    fn set_approval_for_all(approval: bool, operator: Identity);

    /// Returns the total number of tokens which have ever been minted.
    #[storage(read)]
    fn tokens_minted() -> u64;

    /// Transfers ownership of the specified token from one user to another.
    ///
    /// # Arguments
    ///
    /// * `to` - The user which the ownership of the token should be set to.
    /// * `token_id` - The unique identifier of the token which should be transfered.
    ///
    /// # Reverts
    ///
    /// * When the `token_id` does not map to an existing token.
    /// * When the sender is not the owner of the token.
    /// * When the sender is not approved to transfer the token on the owner's behalf.
    /// * When the sender is not approved to transfer all tokens on the owner's behalf.
    #[storage(read, write)]
    fn transfer(to: Identity, token_id: u64);
}
