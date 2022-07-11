library interface;

use std::{contract_id::ContractId, identity::Identity, option::Option};

abi Token {
    #[storage(read, write)]fn mint_to(amount: u64, to: Identity);
    #[storage(read, write)]fn constructor(airdrop_contract: Option<ContractId>, total_supply: u64);
}
