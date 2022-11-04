contract;

dep events;
dep data_structures;
dep errors;
dep interface;
dep utils;

use events::{
    ExternalBalanceTransfer,
    InternalBalanceChanged,
};
use data_structures::{
    UserBalanceOp, 
    UserBalanceOpKind
};
use errors::Error;
use interface::UserBalance;
use utils::{
    handle_remaining_eth,
    validate_user_balance_op,
};

use std::{
    storage::StorageMap,
    address::Address,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::{revert, require},
    token::{force_transfer_to_contract,transfer_to_output},
    vec::Vec,
    option::Option,
    logging::log,
};

use AssetHelpers::{
   _is_eth,
   _as_ierc20,
    translate_to_ierc20,
};

/// use VaultAuthorization::{
///     has_Approved_Relayer
/// };


storage {
    internal_token_balance: StorageMap<(Address, ContractId), u64> = StorageMap {},
    temporarily_pausable_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
}


/// Implement User Balance interactions, which combine Internal Balance and using the Vault's ERC20 allowance.
///
/// Users can deposit tokens into the Vault, where they are allocated to their Internal Balance, and later
/// transferred or withdrawn. It can also be used as a source of tokens when joining Pools, as a destination
/// when exiting them, and as either when performing swaps. This usage of Internal Balance results in greatly reduced
/// gas costs when compared to relying on plain ERC20 transfers, leading to large savings for frequent users.
///
/// Internal Balance management features batching, which means a single contract call can be used to perform multiple
/// operations of different kinds, with different senders and recipients, at once.
impl UserBalance for Contract {
    #[storage(read,write)]
    fn get_internal_balance(user: Address, tokens: Vec<ContractId>) -> Vec<u64>{
        let mut balances:Vec<u64> = ~Vec::new();
        let mut i = 0;
        while i < tokens.len(){
            balances.push(get_internal_balance(user, tokens.get(i).unwrap()));
            i = i + 1;
        }
        return balances;
    }

    #[storage(read, write)]
    fn manage_user_balance(ops: Vec<UserBalanceOp>) {
        /// is_reentrant();
        /// We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
        let mut ethWrapped: u64 = 0;

        /// Cache for these checks so we only perform them once (if at all).
        let checkedCallerIsRelayer = false;
        let mut checkedNotPaused = false;
        let mut i = 0;
        while i < ops.len() {
            let ops :(UserBalanceOpKind, ContractId, u64, Address, Address ) = validate_user_balance_op(
                ops.get(i).unwrap(),
                checkedCallerIsRelayer
            );
            let(kind, asset, amount, sender, recipient, checkedCallerIsRelayer) = ops;
             if let UserBalanceOpKind::WITHDRAW_INTERNAL = kind {
                /// Internal Balance withdrawals can always be performed by an authorized account.
                withdraw_from_internal_balance(asset, sender, recipient, amount);
            } else {
                /// All other operations are blocked if the contract is paused.

                /// We cache the result of the pause check and skip it for other operations in this same transaction
                /// (if any).
                if (!checkedNotPaused) {
                    // let x = abi(TemporarilyPausable, temporarily_pausable_contract_id);
                    // x._ensure_not_paused();
                    checkedNotPaused = true;
                }

                if let UserBalanceOpKind::DEPOSIT_INTERNAL = kind {
                    deposit_to_internal_balance(asset, sender, recipient, amount);

                    // Keep track of all ETH wrapped into WETH as part of a deposit.
                    if (_is_eth(asset)) {
                        ethWrapped = ethWrapped + amount;
                    }
                } else {
                    // Transfers don't support ETH.
                    require(!_is_eth(asset), Error::CANNOT_USE_ETH_SENTINEL);
                    let token = _as_ierc20(asset);

                    if let  UserBalanceOpKind::TRANSFER_INTERNAL = kind {
                        transfer_internal_balance(token, sender, recipient, amount);
                    } else {
                        // TRANSFER_EXTERNAL
                        transfer_to_external_balance(token, sender, recipient, amount);
                    }
                }
            }
        }
        handle_remaining_eth(ethWrapped);
    }

    #[storage(read,write)]
    fn external_receive_asset(asset: ContractId, amount: u64, sender: Address, fromInternalBalance: bool) {
        receive_asset(asset, amount, sender, fromInternalBalance);
    }

    #[storage(read,write)]
    fn external_send_asset(asset: ContractId,amount: u64,recipient: Address,toInternalBalance: bool) {
        send_asset(asset, amount, recipient, toInternalBalance);
    }
}

#[storage(read,write)] 
fn deposit_to_internal_balance(
    asset: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64
)  {
    increase_internal_balance(recipient, translate_to_ierc20(asset), amount);
    receive_asset(asset, amount, sender, false);
}    

#[storage(read,write)] 
fn transfer_internal_balance(
    token: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64
)  {
    // A partial decrease of Internal Balance is disallowed: `sender` must have the full `amount`.
    decrease_internal_balance(sender, token, amount, false);
    increase_internal_balance(recipient, token, amount);
}

#[storage(read,write)] 
fn transfer_to_external_balance(
    token: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64
)  {
    if (amount > 0) {
        transfer_to_output(amount, token, recipient);

        log(ExternalBalanceTransfer {
            amount: amount,
            recipient: recipient,
            sender: sender,
            token: token,
        });
    }
}

#[storage(read,write)] 
fn withdraw_from_internal_balance(
    asset: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64
)  {
    // A partial decrease of Internal Balance is disallowed: `sender` must have the full `amount`.
    decrease_internal_balance(sender, translate_to_ierc20(asset), amount, false);
    send_asset(asset, amount, recipient, false);
}


/// Increases `account`L's Internal Balance for `token` by `amount`.
#[storage(read,write)] 
fn increase_internal_balance(
    account: Address,
    token: ContractId,
    amount: u64
)  {
    let currentBalance: u64 = get_internal_balance(account, token);
    let newBalance: u64 = amount+ currentBalance;
    //todo- When INT256 is implemented
    set_internal_balance(account, token, newBalance, amount);
}

#[storage(read,write)]
fn receive_asset(asset: ContractId, amount: u64, sender: Address, fromInternalBalance: bool) {
    let mut amount = amount;
    if amount == 0 {
        return;
    }

    if _is_eth(asset) {
        require(!fromInternalBalance, Error::INVALID_ETH_INTERNAL_BALANCE);

        /// The ETH amount to receive is deposited into the WETH contract, which will in turn mint WETH for
        /// the Vault at a 1:1 ratio.

        /// A check for this condition is also introduced by the compiler, but this one provides a revert reason.
        /// Note we're checking for the Vault's total balance, *not* ETH sent in this transaction.
        require(balance_of(BASE_ASSET_ID, contract_id())>= amount, Error::INSUFFICIENT_ETH);
        force_transfer_to_contract(amount, BASE_ASSET_ID, contract_id());
    } else {
        let token = _as_ierc20(asset);

        if fromInternalBalance {
            /// We take as many tokens from Internal Balance as possible: any remaining amounts will be transferred.
            let deductedBalance: u64 = decrease_internal_balance(sender, token, amount, true);
            /// Because `deductedBalance` will be always the lesser of the current internal balance
            /// and the amount to decrease, it is safe to perform unchecked arithmetic.
            amount = amount - deductedBalance;
        }

        if amount > 0 {
            force_transfer_to_contract(amount, token, contract_id());
        }
    }
}

/// Sends `amount` of `asset` to `recipient`. If `toInternalBalance` is true, the asset is deposited as Internal
/// Balance instead of being transferred.
///
/// If `asset` is ETH, `toInternalBalance` must be false (as ETH cannot be held as internal balance), and the funds
/// are instead sent directly after unwrapping WETH.
#[storage(read,write)]
fn send_asset(
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
        require(!toInternalBalance, Error::INVALID_ETH_INTERNAL_BALANCE);
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address, _ => revert(0), 
        };

        // First, the Vault withdraws deposited ETH from the WETH contract, by burning the same amount of WETH
        // from the Vault. This receipt will be handled by the Vault's `receive`

        // Then, the withdrawn ETH is sent to the recipient.
        transfer_to_output(amount, BASE_ASSET_ID, sender);
    } else {
        let token = _as_ierc20(asset);
        if (toInternalBalance) {
            increase_internal_balance(recipient, token, amount);
        } else {
            transfer_to_output(amount, token, recipient);
        }
    }
}

/// Decreases `account`'s Internal Balance for `token` by `amount`. If `allowPartial` is true, this function
/// doesn't revert if `account` doesn't have enough balance, and sets it to zero and returns the deducted amount
/// instead.
#[storage(read,write)]
fn decrease_internal_balance(
    account: Address,
    token: ContractId,
    amount: u64,
    allowPartial: bool
) -> u64 {
    let currentBalance: u64 = get_internal_balance(account, token);
    require(allowPartial || (currentBalance >= amount), Error::INSUFFICIENT_INTERNAL_BALANCE);

    let deducted = currentBalance - amount;
    // By construction, `deducted` is lower or equal to `currentBalance`, so we don't need to use checked
    // arithmetic.
    let newBalance: u64 = currentBalance - deducted;
    
        // Todo When signed Integers are added
    set_internal_balance(account, token, newBalance, (deducted));
    return deducted; 
}


/// Sets `account`'s Internal Balance for `token` to `newBalance`.
///
/// Emits an `InternalBalanceChanged` event. This event includes `delta`, which is the amount the balance increased
/// (if positive) or decreased (if negative). To avoid reading the current balance in order to compute the delta,
/// this function relies on the caller providing it directly.
// Todo When signed Integers are added
#[storage(read,write)]
fn set_internal_balance(
    account: Address,
    token: ContractId,
    newBalance: u64,
    delta: u64,
) {
    storage.internal_token_balance.insert((account, token), newBalance);
    log(InternalBalanceChanged{
        account: account,
        delta: delta,
        token: token,
    });
}

#[storage(read,write)]
fn get_internal_balance(account: Address, token: ContractId) -> u64 {
    return storage.internal_token_balance.get((account, token));
}

