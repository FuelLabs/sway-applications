contract;

use std::{call_frames::contract_id, token::mint_to_address};

abi GovToken {
    fn mint_and_send_to_address(amount: u64, recipient: Address) -> bool;
}

impl GovToken for Contract {
    fn mint_and_send_to_address(amount: u64, recipient: Address) -> bool {
        mint_to_address(amount, recipient);
        true
    }
}
