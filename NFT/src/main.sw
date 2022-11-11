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
        is_approved_for_all(operator, owner)
    }

    #[storage(read)]
    fn max_supply() -> Option<u64> {
        max_supply()
    }

    #[storage(read, write)]
    fn mint(amount: u64, to: Identity) {
        mint(amount, to);
    }

    #[storage(read)]
    fn meta_data(token_id: u64) -> Option<TokenMetaData> {
        meta_data(token_id)
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity> {
        owner_of(token_id)
    }

    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>) {
        set_admin(new_admin);
    }

    #[storage(read, write)]
    fn set_approval_for_all(approval: bool, operator: Identity) {
        set_approval_for_all(approval, operator);
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
