contract;

use std::token::mint_to_address;

abi Asset {
    fn mint_and_send_to_address(amount: u64, recipient: Address);
}

impl Asset for Contract {
    fn mint_and_send_to_address(amount: u64, recipient: Address) {
        mint_to_address(amount, recipient);
    }
}
