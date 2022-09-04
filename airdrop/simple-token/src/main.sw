contract;

dep errors;
dep interface;

use errors::{
    AccessError,
    InitError,
    InputError,
};
use interface::SimpleToken;
use std::{
    chain::auth::{
        AuthError,
        msg_sender,
    },
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::require,
    token::mint_to,
};

storage {
    /// The Address or Contract that has permission to mint.
    minter: Identity = Identity::ContractId(~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
    /// The maximum number of tokens ever to be minted.
    token_supply: u64 = 0,
    /// The current number of tokens minted.
    tokens_minted: u64 = 0,
}

impl SimpleToken for Contract {
    #[storage(read, write)]
    fn constructor(minter: Identity, token_supply: u64) {
        // If the token supply is anything other than 0, we know that the constructor has already
        // been called.
        require(storage.token_supply == 0, InitError::AlreadyInitialized);
        require(token_supply != 0, InitError::TokenSupplyCannotBeZero);

        storage.minter = minter;
        storage.token_supply = token_supply;
    }

    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity) {
        // Ensure that the sender is the minter.
        let sender = msg_sender().unwrap();
        require(sender == storage.minter, AccessError::SenderNotPermittedToMint);

        let tokens_minted = storage.tokens_minted;
        require(amount + tokens_minted <= storage.token_supply, InputError::GreaterThanMaximumSupply);

        mint_to(amount, to);
    }
}
