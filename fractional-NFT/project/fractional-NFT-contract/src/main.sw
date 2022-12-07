contract;

dep interface;
dep utils;

use interface::FractionalNFT;
use std::{
    auth::msg_sender,
    call_frames::contract_id,
    context::this_balance,
    token::mint,
};
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
        require(storage.nft.is_none(), "Already initalized error");

        transfer_nft(nft, Identity::ContractId(contract_id()), token_id);
        
        storage.nft = Option::Some(nft);
        storage.owner = Option::Some(owner);
        storage.supply = supply;
        storage.token_id = token_id;

        mint(supply);
    }

    #[storage(read)]
    fn supply() -> u64 {
        storage.supply
    }

    #[storage(read, write)]
    fn withdraw() {
        require(storage.owner.is_some() && msg_sender().unwrap() == storage.owner.unwrap(), "Not NFT owner");
        require(this_balance(contract_id()) == storage.supply, "All tokens not returned");

        transfer_nft(storage.nft.unwrap(), storage.owner.unwrap(), storage.token_id);
        storage.nft = Option::None();
        storage.owner = Option::None();
    }
}
