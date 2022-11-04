library interface;

dep data_structures;

use data_structures::UserBalanceOp;

use std::{
    vec::Vec,
    contract_id::ContractId,
    address::Address,
};

abi UserBalance {
    #[storage(read,write)]fn manage_user_balance(ops: Vec<UserBalanceOp>);
    #[storage(read,write)]fn get_internal_balance(user: Address, tokens: Vec<ContractId>) -> Vec<u64>;
    #[storage(read,write)]fn external_receive_asset(asset: ContractId, amount: u64, sender: Address, fromInternalBalance: bool);
    #[storage(read,write)]fn external_send_asset(asset: ContractId,amount: u64,recipient: Address,toInternalBalance: bool);
}
