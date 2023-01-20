contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::{State, TokenMetadata};
use errors::{InitError, ValidityError};
use interface::Auxiliary;
use std::auth::msg_sender;
use nft::{
    administrator::{
        admin,
        Administrator,
        set_admin,
    },
    approve,
    approved,
    balance_of,
    burnable::{
        burn,
        Burn,
    },
    is_approved_for_all,
    mint,
    NFT,
    owner_of,
    set_approval_for_all,
    supply::{
        max_supply,
        set_max_supply,
        Supply,
    },
    token_metadata::{
        set_token_metadata,
        token_metadata,
    },
    tokens_minted,
    transfer,
};

storage {
    state: State = State::Uninitialized,
}

impl Administrator for Contract {
    #[storage(read)]
    fn admin() -> Option<Identity> {
        admin()
    }

    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>) {
        require(admin().is_some(), ValidityError::NoContractAdmin);
        set_admin(new_admin);
    }
}

impl Burn for Contract {
    #[storage(read, write)]
    fn burn(token_id: u64) {
        set_token_metadata(Option::None::<TokenMetadata>(), token_id);
        burn(token_id);
    }
}

impl Auxiliary for Contract {
    #[storage(read, write)]
    fn constructor(new_admin: Option<Identity>, new_max_supply: Option<u64>) {
        require(storage.state == State::Uninitialized, InitError::CannotReinitialize);
        storage.state = State::Initialized;

        set_admin(new_admin);
        set_max_supply(new_max_supply);
    }

    #[storage(read)]
    fn token_metadata(token_id: u64) -> Option<TokenMetadata> {
        token_metadata(token_id)
    }
}

impl NFT for Contract {
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

    #[storage(read)]
    fn is_approved_for_all(operator: Identity, owner: Identity) -> bool {
        is_approved_for_all(operator, owner)
    }

    #[storage(read, write)]
    fn mint(amount: u64, to: Identity) {
        require(storage.state == State::Initialized, InitError::NotInitialized);
        require(admin().is_none() || (msg_sender().unwrap() == admin().unwrap()), ValidityError::SenderNotAdmin);
        require(max_supply().is_none() || (tokens_minted() + amount <= max_supply().unwrap()), ValidityError::MaxTokensMinted);

        mint(amount, to);

        let mut token_index = tokens_minted() - amount;
        while token_index < tokens_minted() {
            set_token_metadata(Option::Some(TokenMetadata::new()), token_index);
            token_index += 1;
        }
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity> {
        owner_of(token_id)
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

impl Supply for Contract {
    #[storage(read)]
    fn max_supply() -> Option<u64> {
        max_supply()
    }

    #[storage(read, write)]
    fn set_max_supply(supply: Option<u64>) {
        require(storage.state == State::Uninitialized, InitError::CannotReinitialize);
        set_max_supply(supply)
    }
}
