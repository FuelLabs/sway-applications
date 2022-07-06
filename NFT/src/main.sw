contract;

dep interface;
dep data_structures;
dep errors;
dep events;
dep utils;

use interface::NFT;
use data_structures::MetaData;
use errors::{AccessError, ApprovalError, InitError, InputError};
use events::{ApprovalEvent, BurnEvent, MintEvent, OperatorEvent, TransferEvent};
use utils::token_metadata;

use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    storage::StorageMap,
    vec::Vec,
};

storage {
    /// Determines if only the `admin` is allowed to call the mint function.
    /// This is only set on the initalization of the contract.
    access_control: bool,

    /// Stores an `Option` of the `Identity` that has permission to mint if `access_control` is set to true.
    /// Will store `None` if this contract does not have `access_control` set.
    /// Only the `admin` is allowed to change the `admin` of the contract.
    admin: Option<Identity>,

    /// Used for O(1) lookup of the number of tokens owned by each `Identity`.
    /// This increments or decrements when minting, transfering ownership, and burning tokens.
    /// Map(Identity => balance)
    balances: StorageMap<Identity,
    u64>, /// Stores the `Metadata` for each token based on the token's `u64` id
    /// Map(token_id => Metadata)
    meta_data: StorageMap<u64,
    Option<MetaData>>, /// Maps a tuple of (owner, operator) identities and stores whether the
    /// operator is allowed to transfer ALL tokens on the owner's behalf.
    /// Map((owner, operator) => approved)
    operator_approval: StorageMap<(Identity,
    Identity), bool>, /// The total number of tokens that have been minted.
    /// This should only be incremented.
    token_count: u64,

    /// The total supply tokens that can be minted. Can only be set on the initalization of the
    /// contract.
    token_supply: u64,
}

impl NFT for Contract {
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
    #[storage(read, write)]fn constructor(admin: Option<Identity>, access_control: bool, token_supply: u64) {
        // This function can only be called once so if the token supply is already set it has
        // already been called
        require(storage.token_supply == 0, InitError::CannotReinitialize);
        // The number of tokens that can be minted cannot be 0
        require(token_supply != 0, InputError::TokenSupplyCannotBeZero);
        // Access control is set to true but there was no admin given
        require((access_control && admin.is_some()) || admin.is_none(), InitError::AccessControlSetAndAdminIsNone);

        // Store the information given
        storage.access_control = access_control;
        storage.admin = admin;
        storage.token_supply = token_supply;
    }

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
    #[storage(read, write)]fn mint(to: Identity, amount: u64) {
        // The current number of tokens minted plus the amount to be minted cannot be
        // greater than the total supply
        let token_count = storage.token_count;
        let total_mint = token_count + amount;
        require(storage.token_supply >= total_mint, InputError::NotEnoughTokensToMint);

        // Ensure that the sender is the admin if this is a controlled access mint
        require(!storage.access_control || (storage.admin.is_some() && msg_sender().unwrap() == storage.admin.unwrap()), AccessError::SenderDoesNotHaveAccessControl);

        // Mint as many tokens as the sender has asked for
        let mut index = token_count + 1;
        let mut minted_tokens = ~Vec::new::<u64>();
        while index <= total_mint {
            // Create the metadata for this new token with the owner
            let meta_data = ~MetaData::new(Option::None(), to);
            storage.meta_data.insert(index, Option::Some(meta_data));

            // Push to minted tokens Vec
            minted_tokens.push(index);

            // Increment the number of tokens minted in this transaction
            index = index + 1;
        }

        // Increment the token count
        storage.token_count = total_mint;
        // Increase the balance of the new owner
        storage.balances.insert(to, storage.balances.get(to) + amount);

        log(MintEvent {
            owner: to, token_ids: minted_tokens
        });
    }

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
    #[storage(read, write)]fn burn(token_id: u64) {
        // Ensure this is a valid token that has already been minted and exists
        let mut meta_data = token_metadata(storage.meta_data.get(token_id));

        // Ensure the sender owns the token that is provided
        let owner = msg_sender().unwrap();
        require(meta_data.owner == owner, AccessError::SenderNotOwner);

        // Burn this token by setting the `token_id` Metadata mapping to `None`
        storage.meta_data.insert(token_id, Option::None());

        // Reduce the balance of tokens for the owner
        storage.balances.insert(owner, storage.balances.get(owner) - 1);

        // Log the burn event
        log(BurnEvent {
            owner, token_id
        });
    }

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
    #[storage(read, write)]fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        // Make sure the `token_id` maps to an existing token
        let mut meta_data = token_metadata(storage.meta_data.get(token_id));

        // Ensure that the sender is either:
        // 1. The owner of the token
        // 2. Approved for transfer of this `token_id`
        // 3. Or an operator and the token is owned by the owner
        let sender = msg_sender().unwrap();
        let approved = meta_data.approved;
        require(sender == meta_data.owner || (approved.is_some() && sender == approved.unwrap()) || (from == meta_data.owner && storage.operator_approval.get((from, sender))), AccessError::SenderNotOwnerOrApproved);

        // Set the new owner of the token and reset the approved Identity
        meta_data.owner = to;
        meta_data.approved = Option::None();
        storage.meta_data.insert(token_id, Option::Some(meta_data));

        // Decrease the previous owner's balance of tokens
        storage.balances.insert(from, storage.balances.get(from) - 1);

        // Increase the new owner's balance of tokens
        storage.balances.insert(to, storage.balances.get(to) + 1);

        // Log the transfer event
        log(TransferEvent {
            from, sender, to, token_id
        });
    }

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
    #[storage(read, write)]fn approve(approved: Option<Identity>, token_id: u64) {
        // Ensure this is a valid token
        let mut meta_data = token_metadata(storage.meta_data.get(token_id));

        // The owner cannot approve themselves
        require(approved.is_none() || (meta_data.owner != approved.unwrap()), ApprovalError::ApproverCannotBeOwner);

        // Ensure that the sender is the owner of the token to be approved
        let sender = msg_sender().unwrap();
        require(meta_data.owner == sender, AccessError::SenderNotOwner);

        // Set and store the `approved` `Identity`
        meta_data.approved = approved;
        storage.meta_data.insert(token_id, Option::Some(meta_data));

        // Log the approval event
        log(ApprovalEvent {
            owner: sender, approved, token_id
        });
    }

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
    #[storage(read, write)]fn set_approval_for_all(owner: Identity, operator: Identity, allow: bool) {
        // Only the owner is allowed to set an operator for themselves
        require(owner == msg_sender().unwrap(), AccessError::SenderNotOwner);

        // Set the identity to have or not have approval to transfer all tokens owned
        storage.operator_approval.insert((owner, operator), allow);

        // Log the operator event
        log(OperatorEvent {
            owner, operator, allow
        });
    }

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
    #[storage(read, write)]fn set_admin(admin: Option<Identity>) {
        // Ensure that the sender is the admin
        require(storage.admin.is_some() && msg_sender().unwrap() == storage.admin.unwrap(), AccessError::SenderCannotSetAccessControl);

        // Set the new admin
        storage.admin = admin;
    }

    /// Returns an `Option` of an `Identity` containing the specified token's `approved` `Identity`.
    /// If there is no `approved` `Identity`, the function will return `None`.
    /// If the given `u64` token ID does not map to an existing `MetaData`, the function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` ID of the token which the `approved` `Identity` should be returned.
    #[storage(read)]fn approved(token_id: u64) -> Option<Identity> {
        let meta_data = storage.meta_data.get(token_id);

        // If the `u64` id maps to an existing token either return `Some` or `None`
        match meta_data {
            Option::Some(MetaData) => {
                // This token id maps to an existing token
                let meta_data = meta_data.unwrap();
                let approved = meta_data.approved;

                // If there is a `Identity` that is approved, return that `Identity`
                // Otherwise return `None`
                match approved {
                    Option::Some(Identity) => Option::Some(approved.unwrap()), Option::None(Identity) => Option::None(), 
                }
            },
            Option::None(MetaData) => Option::None(), 
        }
    }

    /// Returns a `u64` of the balance of the specified `Identity`.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` of which the balance should be checked.
    #[storage(read)]fn balance_of(owner: Identity) -> u64 {
        storage.balances.get(owner)
    }

    /// Returns a `bool` of whether the `Identity` is approved to transfer all tokens on the `owner`s behalf.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` which has given approval.
    /// * `operator` - The `Identity` which has recieved approval to transfer tokens on the `owner`s behalf.
    #[storage(read)]fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        storage.operator_approval.get((owner, operator))
    }

    /// Returns an `Option` of an `Identity` which owns the specified token id.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` id of the token.
    #[storage(read)]fn owner_of(token_id: u64) -> Option<Identity> {
        let meta_data = storage.meta_data.get(token_id);

        // If the `u64` id maps to an existing token either return `Some` or `None`
        match meta_data {
            Option::Some(MetaData) => {
                // This token id maps to an existing token and return the owner of the token
                let meta_data = meta_data.unwrap();
                Option::Some(meta_data.owner)
            },
            Option::None(MetaData) => Option::None(), 
        }
    }

    /// Returns a `u64` of the total supply of tokens which can be minted for the NFT contract.
    #[storage(read)]fn total_supply() -> u64 {
        storage.token_supply
    }
}
