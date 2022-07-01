contract;

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use abi::NFT;
use data_structures::MetaData;
use errors::{AccessError, ApprovalError, InitError, InputError};
use events::{ApprovalEvent, BurnEvent, MintEvent, OperatorEvent, TransferEvent};
use utils::sender_identity;

use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount, this_balance},
    hash::sha256,
    identity::*,
    logging::log,
    option::*,
    result::*,
    revert::revert,
    storage::StorageMap,
    vec::Vec,
};

storage {
    /// Determines if only the admin is allowed to call the mint function
    /// Only set on the initalization of the contract
    access_control: bool,

    /// Stores the identity that has permission to mint if `access_control` is set
    admin: Identity,

    /// Used of O(1) lookup of the number of tokens owned by each Identity
    /// This increments or decrements when minting, transfer of ownership, and burning of tokens
    /// Map(Identity => balance)
    balances: StorageMap<Identity,u64>, 
    
    /// Stores the Metadata for each token based on token id
    /// Map(token_id => Metadata)
    meta_data: StorageMap<u64,Option<MetaData>>, 
    
    /// Maps a b256 hash of the (owner, operator) identities and stores whether the
    /// operator is allowed to transfer ALL tokens on the owner's behalf
    /// Map(hash => approved)
    operator_approval: StorageMap<b256,bool>, 

    /// The total number of tokens that have been minted
    /// This should only be incremented
    token_count: u64,

    /// The total supply tokens that can be minted. Can only be set
    /// on the initalization of the contract
    token_supply: u64,
}

impl NFT for Contract {
    /// Constructor for the NFT. Calling this function will instantiate the total supply, the admin
    /// `Identity`, and whether access control is enabled. These values can only be set once.
    /// Before this function is called the contract will not allow any minting or transfering of tokens.
    ///
    /// # Arguments
    ///
    /// * `admin` - The `Identity` which has the ability to mint if `access_control` is set and change the contract's admin.
    /// * `access_control` - The `bool` which will determine whether identities will need to approval to mint.
    /// * `token_supply` - The total supply of tokens which will be allowed to mint.
    ///
    /// # Reverts
    ///
    /// * The constructor function has already been called.
    /// * The `token_count` is set to 0.
    #[storage(read, write)]fn constructor(admin: Identity, access_control: bool, token_supply: u64) {
        // This function can only be called once so if the token supply is already set it has
        // already been called
        require(storage.token_supply == 0, InitError::CannotReinitialize);
        // The number of tokens that can be minted cannot be 0
        require(token_supply != 0, InputError::TokenSupplyCannotBeZero);

        // Store the information given
        storage.admin = admin;
        storage.access_control = access_control;
        storage.token_supply = token_supply;
    }

    /// Mints a specified amount of tokens to the given `Identity`.
    ///
    /// # Arguments
    ///
    /// * `to` - The `Identity` which will own the minted tokens.
    /// * `amount` - The number of tokens to be minted.
    ///
    /// # Reverts
    ///
    /// * When the `amount` is set to 0.
    /// * When the sender attempts to mint more tokens than total supply.
    /// * When the sender is not the admin and `access_control` is set.
    #[storage(read, write)]fn mint(to: Identity, amount: u64) {
        // The current number of tokens minted plus the amount to be minted cannot be
        // greater than the total supply
        require(storage.token_supply >= (storage.token_count + amount), InputError::NotEnoughTokensToMint);

        // Ensure that the sender is on the approved mint list if this is a accessed mint
        require(!storage.access_control || sender_identity() == storage.admin, AccessError::SenderDoesNotHaveAccessControl);

        // Mint as many tokens as the sender has asked for
        let mut index = 0;
        let mut minted_tokens: Vec<u64> = ~Vec::new::<u64>();
        while index < amount {
            // Increment the token count
            storage.token_count = storage.token_count + 1;

            // Create the metadata for this new token with the owner
            let meta_data = MetaData {
                owner: to, approved: Option::None()
            };
            storage.meta_data.insert(storage.token_count, Option::Some(meta_data));

            // Increase the balance of the new owner
            storage.balances.insert(to, storage.balances.get(to) + 1);

            // and the number of tokens minted in this transaction
            index = index + 1;

            // Push to minted tokens Vec
            minted_tokens.push(storage.token_count);
        }

        log(MintEvent {
            owner: to, token_ids: minted_tokens
        });
    }

    /// Burns the specified token. When burned, the NFT Metadata of the token is set
    /// to none. After the token has been burned, no one will be able to fetch any data
    /// about this token or have control over it.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token which is to be burned.
    ///
    /// * Reverts
    ///
    /// * When `token_id` does map to an existing token.
    /// * When sender is not the owner of the `token_id`.
    #[storage(read, write)]fn burn(token_id: u64) {
        // Ensure this is a valid token that has already been minted and exists
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        // Ensure the sender owns the token that is provided
        let sender = sender_identity();
        let meta_data: MetaData = meta_data.unwrap();
        require(meta_data.owner == sender, AccessError::SenderNotOwner);

        // Burn this token by setting the `token_id` mapping to `None`
        storage.meta_data.insert(token_id, Option::None());

        // Reduce the balance of tokens for the owner
        storage.balances.insert(sender, storage.balances.get(sender) - 1);

        // Log the burn event
        log(BurnEvent {
            owner: sender, token_id
        });
    }

    /// Transfers ownership of the token from one `Identity` to another.
    ///
    /// # Arguments
    ///
    /// * `from` - The `Identity` which currently owns the token to be transfered.
    /// * `to` - The `Identity` which the ownership of the token should be set to.
    /// * `token_id` - The `u64` id of the token which should be transfered.
    ///
    /// # Reverts
    ///
    /// * When the `token_id` does not map to an existing token.
    /// * When the sender is not the `owner`.
    /// * When the sender is not approved to transfer the `token_id` on the owner's behalf.
    /// * When the sender is not approved to transfer all tokens on the owner's behalf.
    #[storage(read, write)]fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        // Make sure the `token_id` maps to an existing token
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        // Ensure that the sender is either:
        // 1. The owner of the token
        // 2. Approved for transfer of this `token_id`,
        // 3. Or an operator and the token is owned by the owner
        let sender = sender_identity();
        let mut meta_data: MetaData = meta_data.unwrap();
        let approved: Option<Identity> = meta_data.approved;
        require(sender == meta_data.owner || (approved.is_some() && sender == approved.unwrap()) || (from == meta_data.owner && storage.operator_approval.get(sha256(from, sender))), AccessError::SenderNotOwnerOrApproved);

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
            from, to, token_id
        });
    }

    /// Gives approval to the 'to' Identity to transfer the specified token on the owner's behalf.
    ///
    /// # Arguments
    ///
    /// * `approved` - The `Identity` which will be allowed to transfer the token.
    /// * `token_id` - The specific token which the owner is giving approval to.
    /// * `approve` - The `bool` which wil allow or disallow transfers.
    ///
    /// # Reverts
    ///
    /// * When `token_id` does not map to an existing token.
    /// * When the 'to' `Identity` is the owner.
    /// * When the sender is not the owner.
    #[storage(read, write)]fn approve(approved: Identity, token_id: u64, approve: bool) {
        // Ensure this  is a valid token
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        // The owner cannot approve themselves to also be the approver
        let mut meta_data: MetaData = meta_data.unwrap();
        require(meta_data.owner != approved, ApprovalError::ApproverCannotBeOwner);

        // Ensure that the sender is the owner of the token to be approved
        require(meta_data.owner == sender_identity(), AccessError::SenderNotOwner);

        // Set and store the approved Identity to the `to` address or to none
        // based on the `approve` bool
        match approve {
            true => {
                meta_data.approved = Option::Some(approved);
            },
            false => {
                meta_data.approved = Option::None();
            }
        }
        storage.meta_data.insert(token_id, Option::Some(meta_data));

        // Log the approval event
        log(ApprovalEvent {
            owner: sender_identity(), approved, token_id
        });
    }

    /// Gives the `operator` `Identity` approval to transfer any tokens owned by
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
        // Create the hash of the owner and operator
        let hash = sha256(owner, operator);

        // The owner cannot set themself as the operator
        require(owner == sender_identity(), AccessError::SenderNotOwner);

        // Set the identity to have or not have approval to transfer all tokens owned
        storage.operator_approval.insert(hash, allow);

        // Log the operator event
        log(OperatorEvent {
            owner, operator
        });
    }

    /// Changes the contract's `admin` `Identity`.
    ///
    /// # Arguments
    ///
    /// * `admin` - The `Identity` of the new `admin` to be stored
    ///
    /// # Reverts
    ///
    /// * When the sender `Identity` is not the `admin` in storage.
    #[storage(read, write)]fn set_admin(admin: Identity) {
        // Ensure that the sender is allowed to add identities to the approved list
        require(sender_identity() == storage.admin, AccessError::SenderCannotSetAccessControl);

        // Add the provided `minter` Identity to the list of identities that are approved to mint
        storage.admin = admin;
    }

    /// Returns an `Identity` of the approved address for a given token. If the token `MetaData` does not exist
    /// the function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The id of the token which the approved `Identity` should be returned.
    #[storage(read)]fn approved(token_id: u64) -> Option<Identity> {
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);

        match meta_data {
            Option::Some(MetaData) => {
                let meta_data: MetaData = meta_data.unwrap();
                let approved = meta_data.approved;

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

    /// Returns a `bool` of whether the Identity is approved to transfer all tokens on the `owner`s behalf.
    ///
    /// # Arguments
    ///
    /// * `owner` - The `Identity` which has given approval.
    /// * `operator` - The `Identity` which has recieved approval to transfer tokens on the `owner`s behalf.
    #[storage(read)]fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        storage.operator_approval.get(sha256(owner, operator))
    }

    /// Returns the `Identity` which owns the given token id.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The `u64` id of the token.
    #[storage(read)]fn owner_of(token_id: u64) -> Option<Identity> {
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);

        match meta_data {
            Option::Some(MetaData) => {
                let meta_data: MetaData = meta_data.unwrap();
                Option::Some(meta_data.owner)
            },
            Option::None(MetaData) => Option::None(), 
        }
    }

    /// Returns a `u64` of the `total_supply` of tokens which can be minted for the NFT contract.
    #[storage(read)]fn total_supply() -> u64 {
        storage.token_supply
    }
}
