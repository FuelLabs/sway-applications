library interface;

use std::{identity::Identity, option::Option};

abi NFT {
    /// Gives approval to the 'approved' `Identity` to transfer the specified token on the owner's behalf.
    ///
    /// # Arguments
    ///
    /// * `approved` - The `Identity` which will be allowed to transfer the token.
    /// * `token_id` - The `u64` ID of the specific token which the owner is giving approval to.
    /// * `approve` - The `bool` which wil allow or disallow transfers.
    ///
    /// # Reverts
    ///
    /// * When `token_id` does not map to an existing token.
    /// * When the 'approved' `Identity` is the token's owner.
    /// * When the sender is not the token's owner.
    #[storage(read, write)]fn approve(approved: Option<Identity>, token_id: u64);

    /// Returns an `Option` of an `Identity` containing the specified token's `approved` `Identity`.
    /// If there is no `approved` `Identity`, the function will return `None`.
    /// If the given `u64` token ID does not map to an existing `MetaData`, the function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` ID of the token which the `approved` `Identity` should be returned.
    #[storage(read)] fn approved(token_id: u64) -> Option<Identity>;

    /// Returns a `u64` of the balance of the specified `Identity`.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` of which the balance should be checked.
    #[storage(read)]fn balance_of(owner: Identity) -> u64;

    /// Burns the specified token. When burned, the NFT Metadata of the token is set
    /// to `None`. After the token has been burned, no one will be able to fetch any data
    /// about this token or have control over it.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` ID of the token which is to be burned.
    ///
    /// * Reverts
    ///
    /// * When `token_id` does map to an existing token.
    /// * When sender is not the owner of the `token_id`.
    #[storage(read, write)]fn burn(token_id: u64);

    /// Constructor for the NFT. Calling this function will initalize the `total_supply`, the `admin`
    /// `Identity`, and the `access_control` boolean. These values can only be set once.
    /// Before this function is called, the contract is unable to perform any minting or transfering of tokens.
    ///
    /// # Arguments
    ///
    /// * `admin` - The `Identity` which has the ability to mint if `access_control` is set to true and change the contract's admin.
    /// * `access_control` - The `bool` which will determine whether identities will need to approval to mint.
    /// * `token_supply` - The `u64` number representing the total supply of tokens which will be allowed to mint.
    ///
    /// # Reverts
    ///
    /// * When the constructor function has already been called.
    /// * When the `token_supply` is set to 0.
    /// * When `access_control` is set to true and no admin was given.
    #[storage(read, write)]fn constructor(access_control: bool, admin: Option<Identity>, token_supply: u64);

    /// Returns a `bool` of whether the `Identity` is approved to transfer all tokens on the `owner`s behalf.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` which has given approval.
    /// * `operator` - The `Identity` which has recieved approval to transfer tokens on the `owner`s behalf.
    #[storage(read)]fn is_approved_for_all(operator: Identity, owner: Identity) -> bool;

    /// Mints a specified amount of tokens to the given `to` `Identity`. Once a token has been minted,
    /// it can be transfered and burned. Calling this mint function will increment the `total_count`.
    /// If the NFT contract has not yet been initalized, any attempts to mint will fail as the
    /// `total_supply` has not yet been set.
    ///
    /// # Arguments
    ///
    /// * `to` - The `Identity` which will own the minted tokens.
    /// * `amount` - The `u64` number of tokens to be minted in this transaction.
    ///
    /// # Reverts
    ///
    /// * When the sender attempts to mint more tokens than total supply.
    /// * When the sender is not the admin and `access_control` is set.
    #[storage(read, write)]fn mint(amount: u64, to: Identity);

    /// Returns an `Option` of an `Identity` which owns the specified token id.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` id of the token.
    #[storage(read)] fn owner_of(token_id: u64) -> Option<Identity>;

    /// Changes the contract's `admin` `Identity`. This new `admin` will have access to minting if
    /// `access_control` is set to true and be able to change the `admin`.
    ///
    /// # Arguments
    ///
    /// * `admin` - The `Identity` of the new `admin` to be stored.
    ///
    /// # Reverts
    ///
    /// * When the sender `Identity` is not the `admin` in storage.
    #[storage(read, write)]fn set_admin(admin: Option<Identity>);

    /// Gives the `operator` `Identity` approval to transfer ALL tokens owned by
    /// the `owner` `Identity`. This can be dangerous.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` which owns tokens.
    /// * `operator` - The `Identity` which may transfer all tokens owned by the `owner`.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the `owner`.
    #[storage(read, write)]fn set_approval_for_all(approve: bool, operator: Identity, owner: Identity);

    /// Returns a `u64` of the total supply of tokens which can be minted for the NFT contract.
    #[storage(read)]fn total_supply() -> u64;

    /// Transfers ownership of the token from one `Identity` to another. Transfers can occur under
    /// one of three conditions:
    /// 1. The token's owner is transfering the token.
    /// 2. The token's approved is transfering the token.
    /// 3. The token's owner has an operator and is transfering the token.
    ///
    /// # Arguments
    ///
    /// * `from` - The `Identity` which currently owns the token to be transfered.
    /// * `to` - The `Identity` which the ownership of the token should be set to.
    /// * `token_id` - The `u64` ID of the token which should be transfered.
    ///
    /// # Reverts
    ///
    /// * When the `token_id` does not map to an existing token.
    /// * When the sender is not the `owner`.
    /// * When the sender is not approved to transfer the `token_id` on the owner's behalf.
    /// * When the sender is not approved to transfer all tokens on the owner's behalf.
    #[storage(read, write)]fn transfer_from(from: Identity, to: Identity, token_id: u64);
}
