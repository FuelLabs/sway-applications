contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::TokenMetaData;
use errors::{AccessError, InitError, InputError};
use interface::NFT;
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
        // let admin = Option::Some(admin);
        // require(storage.max_supply == 0, InitError::CannotReinitialize);
        // require(max_supply != 0, InputError::TokenSupplyCannotBeZero);
        // require((access_control && admin.is_some()) || (!access_control && admin.is_none()), InitError::AdminIsNone);

        // storage.access_control = access_control;
        // storage.admin = admin;
        // storage.max_supply = max_supply;
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
    fn tokens_minted() -> u64 {
        tokens_minted()
    }

    #[storage(read, write)]
    fn transfer(to: Identity, token_id: u64) {
        transfer(to, token_id);
    }
}
