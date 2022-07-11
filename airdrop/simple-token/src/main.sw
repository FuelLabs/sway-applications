contract;

dep errors;
dep interface;

use errors::{AccessError, InitError, InputError};
use interface::SimpleToken;
use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    result::Result,
    revert::revert,
    token::mint_to,
};

storage {
    /// The airdrop distribution contract that has permission to mint.
    airdrop_contract: Option<ContractId> = Option::None,
    /// The maximum number of tokens ever to be minted.
    token_supply: u64 = 0,
    /// The current number of tokens minted.
    tokens_minted: u64 = 0,
}

impl SimpleToken for Contract {
    #[storage(read, write)]fn constructor(airdrop_contract: Option<ContractId>, token_supply: u64) {
        // If the token supply is anything other than 0, we know that the constructor has already
        // been called.
        require(storage.token_supply == 0, InitError::AlreadyInitialized);
        require(token_supply != 0, InitError::TokenSupplyCannotBeZero);

        storage.airdrop_contract = airdrop_contract;
        storage.token_supply = token_supply;
    }

    #[storage(read, write)]fn mint_to(amount: u64, to: Identity) {
        let airdrop_contract = storage.airdrop_contract;
        let sender = msg_sender().unwrap();

        // Ensure that the sender is the airdrop distributor contract.
        match sender {
            Identity::ContractId(sender) => {
                require(airdrop_contract.is_some() && sender == airdrop_contract.unwrap(), AccessError::SenderNotPermittedToMint);
            }
            _ => revert(0), 
        }

        let tokens_minted = storage.tokens_minted;
        require(amount + tokens_minted <= storage.token_supply, InputError::GreaterThanMaximumSupply);

        mint_to(amount, to);
    }
}
