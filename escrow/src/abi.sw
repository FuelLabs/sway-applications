library abi;

dep data_structures;

use std::{identity::Identity, vec::Vec};
use data_structures::Asset;

abi Escrow {
    #[storage(read, write)] fn create_escrow(assets: Vec<Asset>, users: Vec<Identity>);
    #[storage(read, write)] fn deposit(identifier: u64);
    #[storage(read, write)] fn approve(identifier: u64);
    #[storage(read, write)] fn withdraw(identifier: u64);
}
