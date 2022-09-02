library utils;

dep interface;

use interface::SimpleToken;
use std::{
    constants::ZERO_B256,
    contract_id::ContractId,
    identity::Identity,
};

/// Calls the `mint_to` function in another contract.
///
/// # Arguments
///
/// `amount` - The quantity of tokens to be minted.
/// `to` - The user which the tokens should be given to.
/// `token` - The external token contract which has an implemented `mint_to` function.
pub fn mint_to(amount: u64, to: Identity, token: ContractId) {
    let token_abi = abi(SimpleToken, token.value);
    token_abi.mint_to(amount, to);
}
