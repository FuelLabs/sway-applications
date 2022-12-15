contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::NFTInfo;
use errors::{AccessError, AssetError};
use events::{Deposited, OwnerChanged, Withdraw};
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
    nft_info: Option<NFTInfo> = Option::None,
    supply: u64 = 0,
}

impl FractionalNFT for Contract {
    // Set price, set reserve, 
    #[storage(read, write)]
    fn deposit(
        nft: ContractId,
        owner: Option<Identity>,
        supply: u64,
        token_id: u64,
    ) {
        require(storage.nft_info.is_none(), AccessError::AlreadyInitialized);

        storage.nft_info = Option::Some(NFTInfo::new(nft, owner, token_id));
        storage.supply = supply;

        transfer_nft(nft, Identity::ContractId(contract_id()), token_id);
        mint_to(supply, msg_sender().unwrap());

        log(Deposited {
            nft,
            owner,
            supply,
            token_id,
        });
    }

    #[storage(read)]
    fn nft_info() -> Option<NFTInfo> {
        storage.nft_info
    }

    #[storage(read, write)]
    fn set_owner(new_owner: Option<Identity>) {
        let nft_info = storage.nft_info;
        require(nft_info.is_some(), "No NFT deposited");
        let mut nft_info = nft_info.unwrap();

        require(nft_info.owner.is_some() && msg_sender().unwrap() == nft_info.owner.unwrap(), AccessError::NotNftOwner);

        let previous_owner = nft_info.owner;
        nft_info.owner = new_owner;
        storage.nft_info = Option::Some(nft_info);

        log(OwnerChanged {
            new_owner,
            previous_owner: previous_owner.unwrap(),
        });
    }

    #[storage(read)]
    fn supply() -> u64 {
        storage.supply
    }

    #[storage(read, write)]
    fn withdraw() {
        let nft_info = storage.nft_info;
        require(nft_info.is_some(), "No NFT deposited");
        let mut nft_info = nft_info.unwrap();

        require(nft_info.owner.is_some() && msg_sender().unwrap() == nft_info.owner.unwrap(), AccessError::NotNftOwner);
        require(this_balance(contract_id()) == storage.supply, AssetError::SupplyNotReturned);

        let owner = nft_info.owner;
        nft_info.owner = Option::None();
        storage.nft_info = Option::Some(nft_info);

        transfer_nft(nft_info.nft, owner.unwrap(), nft_info.token_id);

        log(Withdraw {
            nft: nft_info.nft,
            owner: owner.unwrap(),
            token_id: nft_info.token_id,
        });
    }
}
