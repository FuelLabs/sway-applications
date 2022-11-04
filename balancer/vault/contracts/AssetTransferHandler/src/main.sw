library AssetTransferHandler;

use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::{revert, require},
    token::{force_transfer_to_contract,transfer_to_output},
};
use AssetHelpers::{_is_eth, _as_ierc20};

use BalancerErrors::{
    INSUFFICIENT_ETH,
    INVALID_ETH_INTERNAL_BALANCE,
};

fn _receive_asset(asset: ContractId, amount: u64, sender: Address, fromInternalBalance: bool) {
    let mut _amount = amount;
    if _amount == 0 {
        return;
    }

    if _is_eth(asset) {
        require(!fromInternalBalance, INVALID_ETH_INTERNAL_BALANCE);

        // The ETH amount to receive is deposited into the WETH contract, which will in turn mint WETH for
        // the Vault at a 1:1 ratio.

        // A check for this condition is also introduced by the compiler, but this one provides a revert reason.
        // Note we're checking for the Vault's total balance, *not* ETH sent in this transaction.
        require( balance_of(BASE_ASSET_ID, contract_id())>= _amount, INSUFFICIENT_ETH);
        force_transfer_to_contract(_amount, BASE_ASSET_ID, contract_id());
    } else {
        let token = _as_ierc20(asset);

        let mut amount = 0;
        if fromInternalBalance {
            // We take as many tokens from Internal Balance as possible: any remaining amounts will be transferred.
            let deductedBalance: u64 = _decrease_internal_balance(sender, token, _amount, true);
            // Because `deductedBalance` will be always the lesser of the current internal balance
            // and the amount to decrease, it is safe to perform unchecked arithmetic.
            amount = amount - deductedBalance;
        }

        if amount > 0 {
            force_transfer_to_contract(amount, token, contract_id());
        }
    }
}


    /*
    * Sends `amount` of `asset` to `recipient`. If `toInternalBalance` is true, the asset is deposited as Internal
    * Balance instead of being transferred.
    *
    * If `asset` is ETH, `toInternalBalance` must be false (as ETH cannot be held as internal balance), and the funds
    * are instead sent directly after unwrapping WETH.
    */
fn _send_asset(
    asset: ContractId,
    amount: u64,
    recipient: Address,
    toInternalBalance: bool
) {
    if (amount == 0) {
        return;
    }

    if (_is_eth(asset)) {
        // Sending ETH is not as involved as receiving it: the only special behavior is it cannot be
        // deposited to Internal Balance.
        require(!toInternalBalance, INVALID_ETH_INTERNAL_BALANCE);
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };

        // First, the Vault withdraws deposited ETH from the WETH contract, by burning the same amount of WETH
        // from the Vault. This receipt will be handled by the Vault's `receive`

        // Then, the withdrawn ETH is sent to the recipient.
        transfer_to_output(amount, BASE_ASSET_ID, sender);
    } else {
        let token = _as_ierc20(asset);
        if (toInternalBalance) {
            _increase_internal_balance(recipient, token, amount);
        } else {
            transfer_to_output(amount, token, recipient);
        }
    }
}

/*
    * Returns excess ETH back to the contract caller, assuming `amountUsed` has been spent. Reverts
    * if the caller sent less ETH than `amountUsed`.
    *
    * Because the caller might not know exactly how much ETH a Vault action will require, they may send extra.
    * Note that this excess value is returned *to the contract caller* (msg.sender). If caller and e.g. swap sender are
    * not the same (because the caller is a relayer for the sender), then it is up to the caller to manage this
    * returned ETH.
    */
fn _handle_remaining_eth(amountUsed: u64) {
    require(msg_amount() >= amountUsed, INSUFFICIENT_ETH);
    let sender: Result<Identity, AuthError> = msg_sender();
    let sender: Address = match sender.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };

    let excess: u64 = msg_amount() - amountUsed;
    if (excess > 0) {
        // msg.sender.sendValue(excess);
        transfer_to_output(excess, BASE_ASSET_ID, sender);
    }
}

