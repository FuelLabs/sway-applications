library interface;

use std::{
    address::*,
    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
    result::*,
    revert::{revert, require},
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
    identity::Identity,
};

abi BalancerPoolToken {
    // fn allowance(owner: Address, spender: Address) -> u64;
    fn transfer_from(
        sender: Address,
        recipient: Address,
        amount: u64
    ) -> bool;
    fn decrease_allowance(spender: Address, amount: u64) -> bool;
    fn _mint_pool_tokens(recipient: Address, amount: u64);
    fn _burn_pool_tokens(sender: Address, amount: u64);
    
}