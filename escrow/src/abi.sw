library abi;

dep data_structures;

use std::{chain::auth::Sender, contract_id::ContractId};

use data_structures::{Asset, User};

abi Escrow {
    fn constructor(users: [Sender;
    2], assets: [Asset;
    2]) -> bool;
    fn deposit() -> bool;
    fn approve() -> bool;
    fn withdraw() -> bool;
    fn get_balance(asset: ContractId) -> (bool, u64);
    fn get_user_data(user: Sender) -> User;
    fn get_state() -> u64;
}
