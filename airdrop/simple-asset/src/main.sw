contract;

dep errors;
dep interface;

use errors::{AccessError, InitError, InputError};
use interface::SimpleAsset;
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
    /// The maximum quantity of the asset ever to be minted.
    asset_supply: u64 = 0,
    /// The current quantity of the asset minted.
    asset_minted: u64 = 0,
}

impl SimpleAsset for Contract {
    #[storage(read, write)]
    fn constructor(minter: Identity, asset_supply: u64) {
        // If the asset supply is anything other than 0, we know that the constructor has already
        // been called.
        require(storage.asset_supply == 0, InitError::AlreadyInitialized);
        require(asset_supply != 0, InitError::AssetSupplyCannotBeZero);

        storage.minter = minter;
        storage.asset_supply = asset_supply;
    }

    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity) {
        // Ensure that the sender is the minter.
        require(msg_sender().unwrap() == storage.minter, AccessError::SenderNotPermittedToMint);
        require(amount + storage.asset_minted <= storage.asset_supply, InputError::GreaterThanMaximumSupply);

        mint_to(amount, to);
    }
}
