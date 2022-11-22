library interface;

dep data_structures;

use data_structures::TokenMetaData;
use std::{identity::Identity, option::Option};

pub struct AdminEvent {
    /// The user which is now the admin of this contract.
    /// If there is no longer an admin then the `Option` will be `None`.
    admin: Option<Identity>,
}

pub struct ApprovalEvent {
    /// The user that has gotten approval to transfer the specified token.
    /// If an approval was revoked, the `Option` will be `None`.
    approved: Option<Identity>,
    /// The user that has given or revoked approval to transfer his/her tokens.
    owner: Identity,
    /// The unique identifier of the token which the approved may transfer.
    token_id: u64,
}

pub struct BurnEvent {
    /// The user that has burned their token.
    owner: Identity,
    /// The unique identifier of the token which has been burned.
    token_id: u64,
}

pub struct MintEvent {
    /// The owner of the newly minted tokens.
    owner: Identity,
    /// The starting range of token ids that have been minted in this transaction.
    token_id_start: u64,
    /// The total number of tokens minted in this transaction.
    total_tokens: u64,
}

abi NFT {
    /// Returns the current admin for the contract.
    ///
    /// # Reverts
    ///
    /// * When the contract does not have an admin.
    #[storage(read)]
    fn admin() -> Identity;

    /// Gives approval to the `approved` user to transfer a specific token on another user's behalf.
    ///
    /// To revoke approval the approved user should be `None`.
    ///
    /// # Arguments
    ///
    /// * `approved` - The user which will be allowed to transfer the token on the owner's behalf.
    /// * `token_id` - The unique identifier of the token which the owner is giving approval for.
    ///
    /// # Reverts
    ///
    /// * When `token_id` does not map to an existing token.
    /// * When the sender is not the token's owner.
    #[storage(read, write)]
    fn approve(approved: Identity, token_id: u64);

    /// Returns the user which is approved to transfer the given token.
    ///
    /// If there is no approved user or the unique identifier does not map to an existing token,
    /// the function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token which the approved user should be returned.
    ///
    /// # Reverts
    ///
    /// * When there is no approved for the `token_id`.
    #[storage(read)]
    fn approved(token_id: u64) -> Identity;

    /// Returns the balance of the `owner` user.
    ///
    /// # Arguments
    ///
    /// * `owner` - The user of which the balance should be returned.
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;

    /// Burns the specified token.
    ///
    /// When burned, the metadata of the token is removed. After the token has been burned, no one
    /// will be able to fetch any data about this token or have control over it.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token which is to be burned.
    ///
    /// * Reverts
    ///
    /// * When `token_id` does not map to an existing token.
    /// * When sender is not the owner of the token.
    #[storage(read, write)]
    fn burn(token_id: u64);

    /// Sets the inital state and unlocks the functionality for the rest of the contract.
    ///
    /// This function can only be called once.
    ///
    /// # Arguments
    ///
    /// * `access_control` - Determines whether only the admin can call the mint function.
    /// * `admin` - The user which has the ability to mint if `access_control` is set to true and change the contract's admin.
    /// * `max_supply` - The maximum supply of tokens that can ever be minted.
    ///
    /// # Reverts
    ///
    /// * When the constructor function has already been called.
    /// * When the `token_supply` is set to 0.
    /// * When `access_control` is set to true and no admin `Identity` was given.
    #[storage(read, write)]
    fn constructor(access_control: bool, admin: Identity, max_supply: u64);

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
    fn max_supply() -> u64;

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
    /// * When the sender attempts to mint more tokens than total supply.
    /// * When the sender is not the admin and `access_control` is set.
    #[storage(read, write)]
    fn mint(amount: u64, to: Identity);

    /// Returns the metadata for the token specified
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token.
    ///
    /// # Reverts
    ///
    /// * When the `token_id` does not map to an exsiting token.
    #[storage(read)]
    fn meta_data(token_id: u64) -> TokenMetaData;

    /// Returns the user which owns the specified token.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token.
    ///
    /// # Reverts
    ///
    /// * When there is no owner for the `token_id`.
    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity;

    /// Changes the contract's admin.
    ///
    /// This new admin will have access to minting if `access_control` is set to true and be able
    /// to change the contract's admin to a new admin.
    ///
    /// # Arguments
    ///
    /// * `admin` - The user which is to be set as the new admin.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the `admin` in storage.
    #[storage(read, write)]
    fn set_admin(admin: Identity);

    /// Gives the `operator` user approval to transfer ALL tokens owned by the `owner` user.
    ///
    /// This can be dangerous. If a malicous user is set as an operator to another user, they could
    /// drain their wallet.
    ///
    /// # Arguments
    ///
    /// * `approve` - Represents whether the user is giving or revoking operator status.
    /// * `operator` - The user which may transfer all tokens on the owner's behalf.
    #[storage(write)]
    fn set_approval_for_all(approve: bool, operator: Identity);

    /// Returns the total supply of tokens which are currently in existence.
    #[storage(read)]
    fn total_supply() -> u64;

    /// Transfers ownership of the specified token from one user to another.
    ///
    /// Transfers can occur under one of three conditions:
    /// 1. The token's owner is transfering the token.
    /// 2. The token's approved user is transfering the token.
    /// 3. The token's owner has a user set as an operator and is transfering the token.
    ///
    /// # Arguments
    ///
    /// * `from` - The user which currently owns the token to be transfered.
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
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}

pub struct OperatorEvent {
    /// Determines whether approval to transfer all of the owner's tokens has been given or revoked.
    approve: bool,
    /// The user which may or may not transfer all tokens on the owner's behalf.
    operator: Identity,
    /// The user which has given or revoked approval to the operator to transfer all of their
    /// tokens on their behalf.
    owner: Identity,
}

pub struct TransferEvent {
    /// The user which previously owned the token that has been transfered.
    from: Identity,
    // The user that made the transfer. This can be the owner, the approved user, or the operator.
    sender: Identity,
    /// The user which now owns the token that has been transfered.
    to: Identity,
    /// The unique identifier of the token which has transfered ownership.
    token_id: u64,
}
