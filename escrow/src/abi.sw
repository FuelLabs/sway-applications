library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::{Asset, EscrowData, State, User, UserEscrows};

abi Escrow {
    #[storage(read, write)]fn constructor(owner: Identity);
    #[storage(read, write)]fn create_escrow(users: [Identity; 2], assets: [Asset; 2]);
    #[storage(read, write)]fn deposit(identifier: u64);
    #[storage(read, write)]fn approve(identifier: u64);
    #[storage(read, write)]fn withdraw(identifier: u64);
    #[storage(read)]fn user_data(identifier: u64, user: Identity) -> User;
    #[storage(read)]fn user_escrows(user: Identity) -> UserEscrows;
    #[storage(read)]fn escrow_data(identifier: u64) -> EscrowData;
}
