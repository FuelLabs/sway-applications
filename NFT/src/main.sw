contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::{NFTMetadata, State};
use errors::{AccessError, InitError};
use interface::NFT;
use std::auth::msg_sender;
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

storage {
    state: State = State::Uninitialize,
}

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
        set_meta_data(Option::None::<NFTMetadata>(), token_id);
    }

    #[storage(read, write)]
    fn constructor(new_admin: Option<Identity>, new_max_supply: Option<u64>) {
        require(storage.state == State::Uninitialize, InitError::CannotReinitialized);
        storage.state = State::Initialize;

        set_admin(new_admin);
        set_max_supply(new_max_supply);
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
        require(storage.state == State::Initialize, InitError::NotInitialized);
        require(admin().is_none() || (msg_sender().unwrap() == admin().unwrap()), AccessError::SenderNotAdmin);
        require(max_supply().is_none() || (tokens_minted() <= max_supply().unwrap()), AccessError::MaxTokensMinted);
        
        mint(amount, to);
    }

    #[storage(read)]
    fn meta_data(token_id: u64) -> Option<NFTMetadata> {
        meta_data(token_id)
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity> {
        owner_of(token_id)
    }

    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>) {
        require(admin().is_some(), AccessError::NoContractAdmin);
        set_admin(new_admin);
    }

    #[storage(write)]
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
