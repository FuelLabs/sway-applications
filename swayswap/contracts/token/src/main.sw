contract;

dep errors;
dep interface;

use errors::Error;
use interface::Token;
use std::{
    address::*,
    chain::auth::msg_sender,
    context::{
        *,
        call_frames::*,
    },
    contract_id::ContractId,
    revert::require,
    storage::*,
    token::*,
};

const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

storage {
    owner: Identity = Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
    mint_amount: u64 = 0,
    mint_list: StorageMap<Identity, bool> = StorageMap {},
}

impl Token for Contract {
    //////////////////////////////////////
    // Owner methods
    //////////////////////////////////////
    #[storage(read, write)]
    fn initialize(mint_amount: u64, identity: Identity) {
        require(storage.owner == Identity::Address(~Address::from(ZERO_B256)), Error::CannotReinitialize);
        // Start the next message to be signed
        storage.owner = identity;
        storage.mint_amount = mint_amount;
    }

    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        storage.mint_amount = mint_amount;
    }

    #[storage(read)]
    fn mint_coins(mint_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        mint(mint_amount);
    }

    #[storage(read)]
    fn burn_coins(burn_amount: u64) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        burn(burn_amount);
    }

    #[storage(read)]
    fn transfer_coins(coins: u64, identity: Identity) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        transfer(coins, contract_id(), identity);
    }

    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, identity: Identity) {
        require(msg_sender().unwrap() == storage.owner, Error::NotOwner);
        transfer(coins, asset_id, identity);
    }

    //////////////////////////////////////
    // Mint public method
    //////////////////////////////////////
    #[storage(read, write)]
    fn mint() {
        require(storage.mint_amount > 0, Error::MintIsClosed);

        // Enable a address to mint only once
        let sender = msg_sender().unwrap();
        require(storage.mint_list.get(sender) == false, Error::AddressAlreadyMint);

        storage.mint_list.insert(sender, true);
        mint_to(storage.mint_amount, sender);
    }

    //////////////////////////////////////
    // Read-Only methods
    //////////////////////////////////////
    #[storage(read)]
    fn get_mint_amount() -> u64 {
        storage.mint_amount
    }

    fn get_balance() -> u64 {
        balance_of(contract_id(), contract_id())
    }

    fn get_token_balance(asset_id: ContractId) -> u64 {
        balance_of(asset_id, contract_id())
    }
}
