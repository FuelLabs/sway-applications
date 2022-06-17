library abi;

use std::{contract_id::ContractId, identity::Identity, option::Option};

abi NFT {
    #[storage(read, write)]
    fn allow_mint(minter: Identity, allow: bool);
    #[storage(read, write)]
    fn approve(to: Identity, token_id: u64, approve: bool);
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;
    #[storage(read, write)]
    fn burn(token_id: u64);
    #[storage(read, write)]
    fn constructor(owner: Identity, access_control: bool, token_supply: u64);
    // #[storage(read)]
    // fn get_approved(token_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn get_tokens(address: Identity) -> u64;
    #[storage(read)]
    fn get_total_supply() -> u64;
    #[storage(read)]
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    #[storage(read, write)]
    fn mint(to: Identity, amount: u64);
    #[storage(read)]
    // fn owner_of(token_id: u64) -> Option<Identity>;
    #[storage(read, write)]
    fn set_approval_for_all(owner: Identity, operator: Identity, approve: bool);
    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}
