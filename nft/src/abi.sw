library abi;

use std::{identity::Identity, contract_id::ContractId};

abi NFT {
    fn allow_mint(minter: Identity, allow: bool) -> bool;
    fn approve(to: Identity, token_id: u64) -> bool;
    fn balance_of(owner: Identity) -> u64;
    fn burn(token_id: u64) -> bool ;
    fn constructor(owner: Identity, access_control: bool, token_supply: u64, token_price: u64, asset: ContractId) -> bool;
    // fn get_approved(token_id: u64) -> Option<Identity>;
    fn get_tokens(address: Identity) -> u64;
    fn get_total_supply() -> u64;
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    fn mint(to: Identity, amount: u64) -> bool ;
    // fn owner_of(token_id: u64) -> Option<Identity>;
    fn set_approval_for_all(owner: Identity, operator: Identity) -> bool;
    fn transfer_from(from: Identity, to: Identity, token_id: u64) -> bool;
}
