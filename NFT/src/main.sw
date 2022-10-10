contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::TokenMetaData;
use errors::{AccessError, InitError, InputError};
use interface::{AdminEvent, ApprovalEvent, BurnEvent, MintEvent, NFT, OperatorEvent, TransferEvent};
use std::{
    chain::auth::msg_sender,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    revert::require,
    storage::StorageMap,
};

storage {
    /// Determines if only the contract's `admin` is allowed to call the mint function.
    /// This is only set on the initalization of the contract.
    access_control: bool = false,
    /// Stores the user that is permitted to mint if `access_control` is set to true.
    /// Will store `None` if this contract does not have `access_control` set.
    /// Only the `admin` is allowed to change the `admin` of the contract.
    admin: Option<Identity> = Option::None,
    /// Stores the user which is approved to transfer a token based on it's unique identifier.
    /// In the case that no user is approved to transfer a token based on the token owner's behalf,
    /// `None` will be stored.
    /// Map(token_id => approved)
    approved: StorageMap<u64, Option<Identity>> = StorageMap {},
    /// Used for O(1) lookup of the number of tokens owned by each user.
    /// This increments or decrements when minting, transfering ownership, and burning tokens.
    /// Map(Identity => balance)
    balances: StorageMap<Identity, u64> = StorageMap {},
    /// The total supply tokens that can ever be minted.
    /// This can only be set on the initalization of the contract.
    max_supply: u64 = 0,
    /// Stores the `TokenMetadata` for each token based on the token's unique identifier.
    /// Map(token_id => TokenMetadata)
    meta_data: StorageMap<u64, TokenMetaData> = StorageMap {},
    /// Maps a tuple of (owner, operator) identities and stores whether the operator is allowed to
    /// transfer ALL tokens on the owner's behalf.
    /// Map((owner, operator) => approved)
    operator_approval: StorageMap<(Identity, Identity), bool> = StorageMap {},
    /// Stores the user which owns a token based on it's unique identifier.
    /// If the token has been burned then `None` will be stored.
    /// Map(token_id => owner)
    owners: StorageMap<u64, Option<Identity>> = StorageMap {},
    /// The total number of tokens that ever have been minted.
    /// This is used to assign token identifiers when minting. This will only be incremented.
    tokens_minted: u64 = 0,
    /// The number of tokens currently in existence.
    /// This is incremented on mint and decremented on burn. This should not be used to assign
    /// unqiue identifiers due to the decrementation of the value on burning of tokens.
    total_supply: u64 = 0,
}

impl NFT for Contract {
    #[storage(read)]
    fn admin() -> Identity {
        // TODO: Remove this and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        let admin = storage.admin;
        require(admin.is_some(), InputError::AdminDoesNotExist);
        admin.unwrap()
    }

    #[storage(read, write)]
    fn approve(approved: Identity, token_id: u64) {
        // Ensure this is a valid token
        // TODO: Remove this and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        let approved = Option::Some(approved);
        let token_owner = storage.owners.get(token_id);
        require(token_owner.is_some(), InputError::TokenDoesNotExist);

        // Ensure that the sender is the owner of the token to be approved
        let sender = msg_sender().unwrap();
        require(token_owner.unwrap() == sender, AccessError::SenderNotOwner);

        // Set and store the `approved` `Identity`
        storage.approved.insert(token_id, approved);

        log(ApprovalEvent {
            owner: sender,
            approved,
            token_id,
        });
    }

    #[storage(read)]
    fn approved(token_id: u64) -> Identity {
        // TODO: This should be removed and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        // storage.approved.get(token_id)
        let approved = storage.approved.get(token_id);
        require(approved.is_some(), InputError::ApprovedDoesNotExist);
        approved.unwrap()
    }

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        storage.balances.get(owner)
    }

    #[storage(read, write)]
    fn burn(token_id: u64) {
        // Ensure this is a valid token
        let token_owner = storage.owners.get(token_id);
        require(token_owner.is_some(), InputError::TokenDoesNotExist);

        // Ensure the sender owns the token that is provided
        let sender = msg_sender().unwrap();
        require(token_owner.unwrap() == sender, AccessError::SenderNotOwner);

        storage.owners.insert(token_id, Option::None());
        storage.balances.insert(sender, storage.balances.get(sender) - 1);
        storage.total_supply -= 1;

        log(BurnEvent {
            owner: sender,
            token_id,
        });
    }

    #[storage(read, write)]
    fn constructor(access_control: bool, admin: Identity, max_supply: u64) {
        // This function can only be called once so if the token supply is already set it has
        // already been called
        // TODO: Remove this and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        let admin = Option::Some(admin);
        require(storage.max_supply == 0, InitError::CannotReinitialize);
        require(max_supply != 0, InputError::TokenSupplyCannotBeZero);
        require((access_control && admin.is_some()) || (!access_control && admin.is_none()), InitError::AdminIsNone);

        storage.access_control = access_control;
        storage.admin = admin;
        storage.max_supply = max_supply;
    }

    #[storage(read)]
    fn is_approved_for_all(operator: Identity, owner: Identity) -> bool {
        storage.operator_approval.get((owner, operator, ))
    }

    #[storage(read)]
    fn max_supply() -> u64 {
        storage.max_supply
    }

    #[storage(read, write)]
    fn mint(amount: u64, to: Identity) {
        let tokens_minted = storage.tokens_minted;
        let total_mint = tokens_minted + amount;
        // The current number of tokens minted plus the amount to be minted cannot be
        // greater than the total supply
        require(storage.max_supply >= total_mint, InputError::NotEnoughTokensToMint);

        // Ensure that the sender is the admin if this is a controlled access mint
        let admin = storage.admin;
        require(!storage.access_control || (admin.is_some() && msg_sender().unwrap() == admin.unwrap()), AccessError::SenderNotAdmin);

        // Mint as many tokens as the sender has asked for
        let mut index = tokens_minted;
        while index < total_mint {
            // Create the TokenMetaData for this new token
            storage.meta_data.insert(index, ~TokenMetaData::new());
            storage.owners.insert(index, Option::Some(to));
            index += 1;
        }

        storage.balances.insert(to, storage.balances.get(to) + amount);
        storage.tokens_minted = total_mint;
        storage.total_supply += amount;

        log(MintEvent {
            owner: to,
            token_id_start: tokens_minted,
            total_tokens: amount,
        });
    }

    #[storage(read)]
    fn meta_data(token_id: u64) -> TokenMetaData {
        require(token_id < storage.tokens_minted, InputError::TokenDoesNotExist);
        storage.meta_data.get(token_id)
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity {
        // TODO: This should be removed and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        //storage.owners.get(token_id).unwrap()
        let owner = storage.owners.get(token_id);
        require(owner.is_some(), InputError::OwnerDoesNotExist);
        owner.unwrap()
    }

    #[storage(read, write)]
    fn set_admin(admin: Identity) {
        // Ensure that the sender is the admin
        // TODO: Remove this and update function definition to include Option once
        // https://github.com/FuelLabs/fuels-rs/issues/415 is revolved
        let admin = Option::Some(admin);
        let current_admin = storage.admin;
        require(current_admin.is_some() && msg_sender().unwrap() == current_admin.unwrap(), AccessError::SenderCannotSetAccessControl);
        storage.admin = admin;

        log(AdminEvent { admin });
    }

    #[storage(read, write)]
    fn set_approval_for_all(approve: bool, operator: Identity) {
        // Store `approve` with the (sender, operator) tuple
        let sender = msg_sender().unwrap();
        storage.operator_approval.insert((sender, operator, ), approve);

        log(OperatorEvent {
            approve,
            owner: sender,
            operator,
        });
    }

    #[storage(read)]
    fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        // Make sure the `token_id` maps to an existing token
        let token_owner = storage.owners.get(token_id);
        require(token_owner.is_some(), InputError::TokenDoesNotExist);
        let token_owner = token_owner.unwrap();

        // Ensure that the sender is either:
        // 1. The owner of the token
        // 2. Approved for transfer of this `token_id`
        // 3. Has operator approval for the `from` identity and this token belongs to the `from` identity
        let sender = msg_sender().unwrap();
        let approved = storage.approved.get(token_id);
        require(sender == token_owner || (approved.is_some() && sender == approved.unwrap()) || (from == token_owner && storage.operator_approval.get((from, sender, ))), AccessError::SenderNotOwnerOrApproved);

        // Set the new owner of the token and reset the approved Identity
        storage.owners.insert(token_id, Option::Some(to));
        if approved.is_some() {
            storage.approved.insert(token_id, Option::None());
        }

        storage.balances.insert(from, storage.balances.get(from) - 1);
        storage.balances.insert(to, storage.balances.get(to) + 1);

        log(TransferEvent {
            from,
            sender,
            to,
            token_id,
        });
    }
}
