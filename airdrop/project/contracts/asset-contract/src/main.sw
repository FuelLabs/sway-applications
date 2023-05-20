contract;

mod errors;
mod interface;

use ::errors::{AccessError, InitError, InputError};
use ::interface::SimpleAsset;
use std::{auth::msg_sender, token::mint_to};

storage {
    /// The current quantity of the asset minted.
    asset_minted: u64 = 0,
    /// The maximum quantity of the asset ever to be minted.
    asset_supply: u64 = 0,
    /// The Address or Contract that has permission to mint.
    minter: Option<Identity> = Option::None,
}

impl SimpleAsset for Contract {
    #[storage(read, write)]
    fn constructor(asset_supply: u64, minter: Identity) {
        // If the asset supply is anything other than 0, we know that the constructor has already
        // been called.
        require(storage.asset_supply.read() == 0, InitError::AlreadyInitialized);
        require(asset_supply != 0, InitError::AssetSupplyCannotBeZero);

        storage.minter.write(Option::Some(minter));
        storage.asset_supply.write(asset_supply);
    }

    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity) {
        // Ensure that the sender is the minter.
        require(storage.minter.read().is_some() && msg_sender().unwrap() == storage.minter.read().unwrap(), AccessError::SenderNotPermittedToMint);
        require(amount + storage.asset_minted.read() <= storage.asset_supply.read(), InputError::GreaterThanMaximumSupply);

        storage.asset_minted.write(storage.asset_minted.read() + amount);
        mint_to(amount, to);
    }
}
