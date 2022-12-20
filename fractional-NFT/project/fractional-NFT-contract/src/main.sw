contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::NFTInfo;
use errors::{AccessError, AssetError};
use events::{AdminEvent, DepositEvent, WithdrawEvent};
use interface::FractionalNFT;
use std::{
    auth::msg_sender,
    call_frames::contract_id,
    context::this_balance,
    logging::log,
    token::mint_to,
};
use utils::transfer_nft;

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
        nft: ContractId,
        supply: u64,
        token_id: u64,
    ) {
        require(storage.nft_info.is_none(), AccessError::AlreadyInitialized);

        // Store information on this fractionalized NFT
        storage.nft_info = Option::Some(NFTInfo::new(admin, nft, token_id));
        storage.supply = supply;

        // Take ownership of the NFT and mint tokens to the sender
        transfer_nft(nft, Identity::ContractId(contract_id()), token_id);
        mint_to(supply, msg_sender().unwrap());

        log(DepositEvent {
            nft,
            admin,
            supply,
            token_id,
        });
    }

    #[storage(read)]
    fn nft_info() -> Option<NFTInfo> {
        storage.nft_info
    }

    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>) {
        require(storage.nft_info.is_some(), AccessError::NoNftDeposited);
        let mut nft_info = storage.nft_info.unwrap();

        require(nft_info.admin.is_some() && msg_sender().unwrap() == nft_info.admin.unwrap(), AccessError::NotNftAdmin);

        // Store the new admin
        let previous_admin = nft_info.admin;
        nft_info.admin = new_admin;
        storage.nft_info = Option::Some(nft_info);

        log(AdminEvent {
            new_admin,
            previous_admin: previous_admin.unwrap(),
        });
    }

    #[storage(read)]
    fn supply() -> u64 {
        storage.supply
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
        transfer_nft(nft_info.nft, to, nft_info.token_id);

        log(WithdrawEvent {
            nft: nft_info.nft,
            owner: to,
            token_id: nft_info.token_id,
        });
    }
}
