contract;

use std::{
    address::*,
    block::*,
    chain::auth::*,
    context::{
        *,
        call_frames::*,
    },
    contract_id::ContractId,
    identity::Identity,
    result::*,
    revert::revert,
    storage::StorageMap,
};

use swayswap_abi::SwaySwap;

/// Store token ID and exchange contract ID in storage
storage {
    tokens: StorageMap<b256, b256> = StorageMap {},
}

/// Return the sender as an Address or panic
pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

impl SwaySwap for Contract {
    #[storage(write)]
    fn add_exchange_contract(token_id: ContractId, exchange_id: ContractId) {
        storage.tokens.insert(token_id, exchange_id);
    }
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId {
        ~ContractId::from(storage.tokens.get(token_id))
    }
}
