contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::TokenMetaData;
use errors::{AccessError, InitError, InputError};
use interface::NFT;
use std::{auth::msg_sender, logging::log, storage::StorageMap};
use sway_libs::nft::{
    administrator::{
        admin,
        set_admin,
    },
    approve,
    approved,
    balance_of,
    burnable::burn,
    is_approved_for_all,
    meta_data::{
        meta_data,
        set_meta_data,
    },
    mint,
    owner_of,
    set_approval_for_all,    
    supply::{
        max_supply,
        set_max_supply,
    },
    tokens_minted,
    transfer,
};

impl NFT for Contract {
    #[storage(read)]
    fn admin() -> Option<Identity> {
        admin()
    }

    #[storage(read, write)]
    fn approve(approved_identity: Option<Identity>, token_id: u64) {
        approve(approved_identity, token_id);
    }

    #[storage(read)]
    fn approved(token_id: u64) -> Option<Identity> {
        approved(token_id)
    }

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        balance_of(owner)
    }

    #[storage(read, write)]
    fn burn(token_id: u64) {
        burn(token_id);
    }

    #[storage(read, write)]
    fn constructor(access_control: bool, admin: Identity, max_supply: u64) {
        // This function can only be called once so if the token supply is already set it has
        // already been called
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
        storage.operator_approval.get((owner, operator))
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
            storage.meta_data.insert(index, TokenMetaData::new());
            storage.owners.insert(index, Option::Some(to));
            index += 1;
        }

        storage.balances.insert(to, storage.balances.get(to) + amount);
        storage.tokens_minted = total_mint;
        storage.total_supply += amount;
    }

    #[storage(read)]
    fn meta_data(token_id: u64) -> TokenMetaData {
        require(token_id < storage.tokens_minted, InputError::TokenDoesNotExist);
        storage.meta_data.get(token_id)
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity {
        //storage.owners.get(token_id).unwrap()
        let owner = storage.owners.get(token_id);
        require(owner.is_some(), InputError::OwnerDoesNotExist);
        owner.unwrap()
    }

    #[storage(read, write)]
    fn set_admin(admin: Identity) {
        // Ensure that the sender is the admin
        let admin = Option::Some(admin);
        let current_admin = storage.admin;
        require(current_admin.is_some() && msg_sender().unwrap() == current_admin.unwrap(), AccessError::SenderCannotSetAccessControl);
        storage.admin = admin;
    }

    #[storage(read, write)]
    fn set_approval_for_all(approve: bool, operator: Identity) {
        // Store `approve` with the (sender, operator) tuple
        let sender = msg_sender().unwrap();
        storage.operator_approval.insert((sender, operator), approve);

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
        require(sender == token_owner || (approved.is_some() && sender == approved.unwrap()) || (from == token_owner && storage.operator_approval.get((from, sender))), AccessError::SenderNotOwnerOrApproved);

        // Set the new owner of the token and reset the approved Identity
        storage.owners.insert(token_id, Option::Some(to));
        if approved.is_some() {
            storage.approved.insert(token_id, Option::None::<Identity>());
        }

        storage.balances.insert(from, storage.balances.get(from) - 1);
        storage.balances.insert(to, storage.balances.get(to) + 1);
    }
}
