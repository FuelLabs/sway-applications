library utils;

dep data_structures;
dep errors;

use data_structures::{
    UserBalanceOpKind,
    UserBalanceOp,
};
use errors::Error;
use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::{revert, require},
    token::transfer_to_output,
    option::Option,
};

/// Returns excess ETH back to the contract caller, assuming `amountUsed` has been spent. Reverts
/// if the caller sent less ETH than `amountUsed`.
///
/// Because the caller might not know exactly how much ETH a Vault action will require, they may send extra.
/// Note that this excess value is returned *to the contract caller* (msg.sender). If caller and e.g. swap sender are
/// not the same (because the caller is a relayer for the sender), then it is up to the caller to manage this
/// returned ETH.
pub fn handle_remaining_eth(amountUsed: u64) {
    require(msg_amount() >= amountUsed, Error::INSUFFICIENT_ETH);

    let excess: u64 = msg_amount() - amountUsed;
    if (excess > 0) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        transfer_to_output(excess, contract_id(), sender);
        // msg.sender.sendValue(excess);
    }
}
