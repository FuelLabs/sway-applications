contract;

dep interface;

use interface::Token;
use std::{
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    token::mint_to,
};

storage {
    airdrop_contract: Option<ContractId> = Option::None,
    token_supply: u64 = 0,
}

impl Token for Contract {
    fn mint_to(amount: u64, to: Identity) {
        
    }
}
