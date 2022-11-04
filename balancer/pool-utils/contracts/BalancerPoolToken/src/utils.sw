library utils;

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

const _VAULT: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b; 

// IVault vaule type is unknown. so for now I am defining it as b256
pub fn get_vault() -> b256 {
    _VAULT
}


/**
    * @dev Override to grant the Vault infinite allowance, causing for Pool Tokens to not require approval.
    *
    * This is sound as the Vault already provides authorization mechanisms when initiation token transfers, which this
    * contract inherits.
*/
pub fn allowance(owner: Address, spender: Address) -> u64 {
    if (spender == ~Address::from(get_vault())) {
        // -1
        0
    } else {
        // return super.allowance(owner, spender);
        // allowance(owner, spender)
        0
    }
}
