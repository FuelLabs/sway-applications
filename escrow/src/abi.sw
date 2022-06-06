library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{Asset, User};

abi Escrow {
    fn constructor(users: [Identity; 2], assets: [Asset; 2]);
    fn deposit();
    fn approve();
    fn withdraw();
    fn get_balance(asset: ContractId) -> (bool, u64);
    fn get_user_data(user: Identity) -> User;
    fn get_state() -> u64;
}
