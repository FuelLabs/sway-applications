library interface;

dep data_structures;

use std::{address::Address, b512::B512, contract_id::ContractId, identity::Identity, vec::Vec};
use data_structures::{Owner, User};

abi MultiSignatureWallet {
    #[storage(read, write)]fn constructor(users: Vec<User>, threshold: u64);

    #[storage(read, write)]fn execute_transaction(to: Identity, value: u64, data: Vec<u64>, signatures: Vec<B512>);

    #[storage(read, write)]fn transfer(to: Identity, asset_id: ContractId, value: u64, data: Vec<u64>, signatures: Vec<B512>);

    #[storage(read)]fn owner(owner: Address) -> Owner;

    #[storage(read)]fn nonce() -> u64;

    fn balance(asset_id: ContractId) -> u64;

    fn transaction_hash(to: Identity, value: u64, data: Vec<u64>, nonce: u64) -> b256;
}
