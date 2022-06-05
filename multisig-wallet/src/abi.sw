library abi;

dep data_structures;

use std::{
    address::Address,
    b512::B512,
    chain::auth::Sender,
    contract_id::ContractId,
};

use data_structures::User;

abi MultiSignatureWallet {
    fn constructor(users: [User; 2], threshold: u64);
    fn execute_transaction(to: Sender, value: u64, data: b256, signatures: [B512; 2]);
    fn transfer(to: Sender, asset_id: ContractId, value: u64, data: b256, signatures: [B512; 2]);
    fn is_owner(owner: Address) -> bool;
    fn balance(asset_id: ContractId) -> u64;
    fn get_transaction_hash(to: Sender, value: u64, data: b256, nonce: u64) -> b256;
    fn nonce() -> u64;
}
