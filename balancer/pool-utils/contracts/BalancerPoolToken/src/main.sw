contract;

dep interface;
dep utils;

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

use interface::BalancerPoolToken;
use utils::{ get_vault, allowance };


impl BalancerPoolToken for Contract {
    // IVault private immutable _VAULT;


    // Overrides


    /**
     * @dev Override to allow for 'infinite allowance' and let the token owner use `transferFrom` with no self-allowance
     */
    fn transfer_from(
        sender: Address,
        recipient: Address,
        amount: u64
    ) -> bool {
        let sender_addr: Result<Identity, AuthError> = msg_sender();
        let sender_addr: Address = match sender_addr.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        let currentAllowance = allowance(sender, sender_addr);
        require(sender_addr == sender || currentAllowance >= amount, "ERC20_TRANSFER_EXCEEDS_ALLOWANCE");

        // _transfer(sender, recipient, amount);
        transfer_to_output(amount, contract_id(), recipient);

        // if (addr != sender && currentAllowance != -1) {
        //     // Because of the previous require, we know that if addr != sender then currentAllowance >= amount
        //     _approve(sender, addr, currentAllowance - amount);
        // }

        true
    }

    /**
     * @dev Override to allow decreasing allowance by more than the current amount (setting it to zero)
     */
    fn decrease_allowance(spender: Address, amount: u64) -> bool {
        let sender_addr: Result<Identity, AuthError> = msg_sender();
        let sender_addr: Address = match sender_addr.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        let currentAllowance = allowance(sender_addr, spender);

        // if (amount >= currentAllowance) {
        //     _approve(sender_addr, spender, 0);
        // } else {
        //     // No risk of underflow due to if condition
        //     _approve(sender_addr, spender, currentAllowance - amount);
        // }

        true
    }

    // Internal fns

    fn _mint_pool_tokens(recipient: Address, amount: u64) {
        mint(amount);
        transfer_to_output(amount, contract_id(), recipient);
    }

    fn _burn_pool_tokens(sender: Address, amount: u64) {
        burn(amount);
    }
}
