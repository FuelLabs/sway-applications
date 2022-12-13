contract;

dep errors;
dep events;
dep interface;
dep utils;

use errors::{AccessError, AssetError};
use events::{Deposited, OwnerChanged, Withdraw};
use interface::FractionalNFT;
use std::{auth::msg_sender, call_frames::contract_id, context::this_balance, logging::log, token::mint_to};
use utils::transfer_nft;

storage {
    nft: Option<ContractId> = Option::None,
    owner: Option<Identity> = Option::None,
    supply: u64 = 0,
    token_id: u64 = 0,
}

impl FractionalNFT for Contract {
    // Set price, set reserve, 
    #[storage(read, write)]
    fn deposit(nft: ContractId, owner: Identity, supply: u64, token_id: u64) {
        require(storage.owner.is_none(), AccessError::AlreadyInitialized);

        storage.nft = Option::Some(nft);
        storage.owner = Option::Some(owner);
        storage.supply = supply;
        storage.token_id = token_id;

        transfer_nft(nft, Identity::ContractId(contract_id()), token_id);
        mint_to(supply, msg_sender().unwrap());

        log(Deposited {nft, owner, supply, token_id});
    }

    #[storage(read)]
    fn nft() -> (Option<ContractId>, u64) {
        (storage.nft, storage.token_id)
    }

    #[storage(read)]
    fn owner() -> Option<Identity> {
        storage.owner
    }

    #[storage(read, write)]
    fn set_owner(new_owner: Option<Identity>) {
        let owner = storage.owner;
        require(owner.is_some() && msg_sender().unwrap() == owner.unwrap(), AccessError::NotNftOwner);
        storage.owner = new_owner;

        log (OwnerChanged{new_owner, previous_owner: owner.unwrap()});
    }

    #[storage(read)]
    fn supply() -> u64 {
        storage.supply
    }

    #[storage(read, write)]
    fn withdraw() {
        require(storage.owner.is_some() && msg_sender().unwrap() == storage.owner.unwrap(), AccessError::NotNftOwner);
        require(this_balance(contract_id()) == storage.supply, AssetError::SupplyNotReturned);

        let nft = storage.nft;
        let owner = storage.owner;
        let token_id = storage.token_id;

        storage.nft = Option::None();
        storage.owner = Option::None();

        transfer_nft(nft.unwrap(), owner.unwrap(), token_id);

        log (Withdraw {nft: nft.unwrap(), owner: owner.unwrap(), token_id});
    }
}
