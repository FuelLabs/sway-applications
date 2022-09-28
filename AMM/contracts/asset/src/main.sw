contract;

dep errors;
dep interface;

use errors::Error;
use interface::Asset;
use std::{
    chain::auth::{
        AuthError,
        msg_sender,
    },
    constants::ZERO_B256,
    context::{
        balance_of,
        call_frames::contract_id,
    },
    prelude::*,
    storage::StorageMap,
    token::{
        burn,
        mint,
        mint_to,
        transfer,
    },
};

storage {
    /// The owner of contract
    owner: Identity = Identity::Address(~Address::from(ZERO_B256)),
    /// The amount to mint
    mint_amount: u64 = 0,
    /// Information describing whether an identity has mint
    mint_list: StorageMap<Identity, bool> = StorageMap {},
}

impl Asset for Contract {
    fn asset_balance(asset_id: ContractId) -> u64 {
        balance_of(asset_id, contract_id())
    }

    fn balance() -> u64 {
        balance_of(contract_id(), contract_id())
    }

    #[storage(read)]
    fn burn_coins(burn_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        burn(burn_amount);
    }

    #[storage(read, write)]
    fn initialize(identity: Identity, mint_amount: u64) {
        require(storage.owner == Identity::Address(~Address::from(ZERO_B256)), Error::CannotReinitialize);
        storage.owner = identity;
        storage.mint_amount = mint_amount;
    }

    #[storage(read, write)]
    fn mint() {
        require(storage.mint_amount > 0, Error::MintIsClosed);

        let sender = msg_sender().unwrap();
        require(storage.mint_list.get(sender) == false, Error::IdentityAlreadyMint);

        storage.mint_list.insert(sender, true);
        mint_to(storage.mint_amount, sender);
    }

    #[storage(read)]
    fn mint_amount() -> u64 {
        storage.mint_amount
    }

    #[storage(read)]
    fn mint_coins(mint_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        mint(mint_amount);
    }

    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        storage.mint_amount = mint_amount;
    }

    #[storage(read)]
    fn transfer_asset_to_output(asset_id: ContractId, amount: u64, identity: Identity) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        transfer(amount, asset_id, identity);
    }

    #[storage(read)]
    fn transfer_coins(amount: u64, identity: Identity) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        transfer(amount, contract_id(), identity);
    }
}
