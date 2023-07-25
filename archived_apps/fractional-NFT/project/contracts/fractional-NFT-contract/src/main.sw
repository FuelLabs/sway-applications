contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use ::data_structures::NFTInfo;
use ::errors::{AccessError, AssetError};
use ::events::{AdminEvent, DepositEvent, WithdrawEvent};
use ::interface::{FractionalNFT, Info};
use std::{auth::msg_sender, call_frames::contract_id, context::this_balance, token::mint_to};
use ::utils::transfer_nft;

storage {
    /// Stores the relevant information on the NFT that is locked in the contract.
    nft_info: Option<NFTInfo> = Option::None,
    /// The total number of tokens that shall ever be minted.
    supply: u64 = 0,
}

impl FractionalNFT for Contract {
    #[storage(read, write)]
    fn deposit(
        admin: Option<Identity>,
        asset_id: ContractId,
        supply: u64,
        token_id: u64,
    ) {
        require(storage.nft_info.is_none(), AccessError::AlreadyInitialized);

        // Store information on this fractionalized NFT
        storage.nft_info = Option::Some(NFTInfo::new(admin, asset_id, token_id));
        storage.supply = supply;

        // Take ownership of the NFT and mint tokens to the sender
        transfer_nft(asset_id, Identity::ContractId(contract_id()), token_id);
        mint_to(supply, msg_sender().unwrap());

        log(DepositEvent {
            admin,
            asset_id,
            supply,
            token_id,
        });
    }

    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>) {
        require(storage.nft_info.is_some(), AccessError::NoNftDeposited);
        let mut nft_info = storage.nft_info.unwrap();

        require(nft_info.admin.is_some() && msg_sender().unwrap() == nft_info.admin.unwrap(), AccessError::NotNftAdmin);

        let previous_admin = nft_info.admin;
        // Store the new admin
        nft_info.admin = new_admin;
        storage.nft_info = Option::Some(nft_info);

        log(AdminEvent {
            new_admin,
            previous_admin: previous_admin.unwrap(),
        });
    }

    #[storage(read, write)]
    fn withdraw(to: Identity) {
        require(storage.nft_info.is_some(), AccessError::NoNftDeposited);
        let mut nft_info = storage.nft_info.unwrap();

        require(nft_info.admin.is_some() && msg_sender().unwrap() == nft_info.admin.unwrap(), AccessError::NotNftAdmin);
        require(this_balance(contract_id()) == storage.supply, AssetError::SupplyNotReturned);

        // Set the contract to have no admin such that it becomes locked
        nft_info.admin = Option::None;
        storage.nft_info = Option::Some(nft_info);

        // Change ownership of the NFT to the `to` identity
        transfer_nft(nft_info.asset_id, to, nft_info.token_id);

        log(WithdrawEvent {
            asset_id: nft_info.asset_id,
            owner: to,
            token_id: nft_info.token_id,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn nft_info() -> Option<NFTInfo> {
        storage.nft_info
    }

    #[storage(read)]
    fn supply() -> u64 {
        storage.supply
    }
}
