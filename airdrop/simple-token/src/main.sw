contract;

dep errors;
dep interface;

use errors::{InitError, AccessError};
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
    airdrop_contract: Option<ContractId> = Option::None,
    token_supply: u64 = 0,
}

impl SimpleToken for Contract {
    #[storage(read, write)]fn constructor(airdrop_contract: Option<ContractId>, token_supply: u64) {
        require(storage.token_supply == 0, InitError::AlreadyInitialized);
        require(token_supply != 0, InitError::TokenSupplyCannotBeZero);

        storage.airdrop_contract = airdrop_contract;
        storage.token_supply = token_supply;
    }

    #[storage(read, write)]fn mint_to(amount: u64, to: Identity) {
        let airdrop_contract = storage.airdrop_contract;
        let sender = msg_sender().unwrap();

        match sender {
            Identity::ContractId(sender) => {
                require(airdrop_contract.is_some() && sender == airdrop_contract.unwrap(), AccessError::SenderNotPermittedToMint);
            }
            _ => revert(0), 
        }

        mint_to(amount, to);
    }
}
