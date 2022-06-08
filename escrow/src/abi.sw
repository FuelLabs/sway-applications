library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{Asset, EscrowData, State, User, UserEscrows};

abi Escrow {
    fn constructor(owner: Identity);
    fn create_escrow(users: [Identity; 2], assets: [Asset; 2]);
    fn deposit(identifier: u64);
    fn approve(identifier: u64);
    fn withdraw(identifier: u64);
    fn user_data(identifier: u64, user: Identity) -> User;
    fn user_escrows(user: Identity) -> UserEscrows;
    fn escrow_data(identifier: u64) -> EscrowData;
}
