library swayswap_abi;

use std::{chain::auth::{AuthError, msg_sender}, contract_id::ContractId, result::Result};

/// Return the sender as an Address or panic
pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

abi SwaySwap {
    // Add exchange contract to the token
    #[storage(write)]
    fn add_exchange_contract(token_id: ContractId, exchange_id: ContractId);
    // Get exchange contract for desired token
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId;
}
