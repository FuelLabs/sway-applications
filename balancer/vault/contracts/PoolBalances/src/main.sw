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
    UserBalanceOpKind,
    PoolSpecialization,
    JoinPoolRequest,
    ExitPoolRequest,
    PoolBalanceChangeKind,
    PoolBalanceChange,
};
use errors::Error;
use interface::PoolBalances;
use utils::{
    get_pool_specialization,
    handle_remaining_eth,
    _validate_tokens_and_get_balances,
    _unsafe_cast_to_int256,
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
    constants::ZERO_B256,
};

use InputHelpers::{
    ensure_input_length_match,
    ensure_input_length_match_second,
};
use AssetHelpers::{
    _is_eth,
    _as_ierc20,
    translate_to_ierc20,
    translate_to_ierc20_second,
};
use BalanceAllocation::{
    increase_cash,
    decrease_cash,
    totals_and_last_change_block
};

// use PoolTokens::_get_pool_tokens;
// use PoolRegistry::_get_pool_specialization;
// use TwoTokenPoolsBalance::_set_two_token_pool_cash_balances;
// use MinimalSwapInfoPoolsBalance::_set_minimal_swap_info_pool_balances;
// use GeneralPoolsBalance::_set_general_pool_balances;
// use ProtocolFeesCollector::ProtocolFeesCollector; 
// use UserBalance::UserBalance;

storage {
    internal_token_balance: StorageMap<(Address, ContractId), u64> = StorageMap {},
    temporarily_pausable_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    protocol_fees_collector_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    pool_tokens_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    pool_registry_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    two_pool_token_balance_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    minimal_swap_info_pools_balance_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    general_pools_balance_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
    user_balance_contract_id: b256 = 0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861,
}

// Stores the Asset Managers (by Pool and token), and implements the top level Asset Manager and Pool interfaces,
// such as registering and deregistering tokens, joining and exiting Pools, and informational functions like `getPool`
// and `getPoolTokens`, delegating to specialization-specific functions as needed.
//
// `managePoolBalance` handles all Asset Manager interactions.
impl PoolBalances for Contract {
    fn join_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest
    ) {
        // This fn doesn't have the nonReentrant modifier: it is applied to `_join_or_exit` instead.

        // Note that `recipient` is not actually payable in the context of a join - we cast it because we handle both
        // joins and exits at once.
        // _join_or_exit(PoolBalanceChangeKind::JOIN, poolId, sender, recipient, to_pool_balance_change(request));
        true;
    }

    fn exit_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: ExitPoolRequest
    ) {
        // // This fn doesn't have the nonReentrant modifier: it is applied to `_join_or_exit` instead.
        // _join_or_exit(PoolBalanceChangeKind::EXIT, poolId, sender, recipient, to_pool_balance_change(request));
        true;
    }
}

// // Converts a JoinPoolRequest into a PoolBalanceChange, with no runtime cost.
// fn to_pool_balance_change(request: JoinPoolRequest) -> PoolBalanceChange {
//     // solhint-disable-next-line no-inline-assembly
//     asm {
//         change := request
//     }
//     return change;
// }

// Converts an ExitPoolRequest into a PoolBalanceChange, with no runtime cost.
// fn _to_pool_balance_change(request: ExitPoolRequest) -> PoolBalanceChange {
//     // solhint-disable-next-line no-inline-assembly
//     assembly {
//         change := request
//     }
    
//     return change;
// }

// Implements both `join_pool` and `exit_pool`, based on `kind`.
#[storage(read, write)]
fn _join_or_exit(
    kind: PoolBalanceChangeKind,
    poolId: b256,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange
) {
    // This fn uses a large number of stack variables (poolId, sender and recipient, balances, amounts, fees,
    // etc.), which leads to 'stack too deep' issues. It relies on private functions with seemingly arbitrary
    // interfaces to work around this limitation.

    ensure_input_length_match(change.assets.len(), change.limits.len());

    // We first check that the caller passed the Pool's registered tokens in the correct order, and retrieve the
    // current balance for each.
    let tokens = translate_to_ierc20_second(change.assets);
    let balances = _validate_tokens_and_get_balances(poolId, tokens);

    // The bulk of the work is done here: the corresponding Pool hook is called, its final balances are computed,
    // assets are transferred, and fees are paid.
    let(finalBalances, amountsInOrOut, paidProtocolSwapFeeAmounts) = _call_pool_balance_change(kind, poolId, sender, recipient, change, balances);

    // // All that remains is storing the new Pool balances.
    // let specialization: PoolSpecialization = get_pool_specialization(poolId);
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     let x = abi(TwoTokenPoolsBalance, storage.two_pool_token_balance_contract_id);
    //     x.set_two_token_pool_cash_balances(poolId, tokens.get(0).unwrap(), finalBalances.get(0).unwrap(), tokens.get(1).unwrap(), finalBalances.get(1).unwrap());
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     let x = abi(MinimalSwapInfoPoolsBalance, storage.minimal_swap_info_pools_balance_contract_id);
    //     x.set_minimal_swap_info_pool_balances(poolId, tokens, finalBalances);
    // } else {
    //     // PoolSpecialization.GENERAL
    //     let x = abi(GeneralPoolsBalances, storage.general_pools_balance_contract_id);
    //     x.set_general_pool_balances(poolId, finalBalances);
    // }

    // Amounts in are positive, out are negative
    let mut positive: bool = false; 
    if let PoolBalanceChangeKind::JOIN = kind {
        positive = true;
    }
    // emit PoolBalanceChanged(
    //         poolId,
    //         sender,
    //         tokens,
    //         // We can unsafely cast to int256 because balances are actually stored as uint112
    //         _unsafeCastToInt256(amountsInOrOut, positive),
    //         paidProtocolSwapFeeAmounts
    // );
}

// Calls the corresponding Pool hook to get the amounts in/out plus protocol fee amounts, and performs the
// associated token transfers and fee payments, returning the Pool's final balances.
#[storage(read, write)]
fn _call_pool_balance_change(
    kind: PoolBalanceChangeKind,
    poolId: b256,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>
) -> (Vec<b256>, Vec<u64>, Vec<u64>)
{
    let(totalBalances, lastChangeBlock) = totals_and_last_change_block(balances);

    // let pool = IBasePool(_getPoolAddress(poolId));
    // // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
    // // let get_protocol_swap_fee_percentage = 1;
    // // todo dummy for now
    // let protocol_swap_fee_percentage = 1;
    // if let PoolBalanceChangeKind::JOIN = kind{
    //     let(amountsInOrOut, dueProtocolFeeAmounts) = 
    //         pool.onJoinPool(
    //         poolId,
    //         sender,
    //         recipient,
    //         totalBalances,
    //         lastChangeBlock,
    //         protocol_swap_fee_percentage,
    //         change.userData
    //     );
    //     ensure_input_length_match_second(balances.len(), amountsInOrOut.len(), dueProtocolFeeAmounts.len());
    //     // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
    //     // their participation.
    //     if let PoolBalanceChangeKind::JOIN = kind {
    //         let finalBalances = _process_join_pool_transfers(sender, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
    //         return (finalBalances, amountsInOrOut, dueProtocolFeeAmounts);
    //     }
    //     else {
    //         let finalBalances = _process_exit_pool_transfers(recipient, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
    //         return (finalBalances, amountsInOrOut, dueProtocolFeeAmounts);
    //     }   
    // }
    // else {
    //     let(amountsInOrOut, dueProtocolFeeAmounts) = 
    //         pool.onExitPool(
    //         poolId,
    //         sender,
    //         recipient,
    //         totalBalances,
    //         lastChangeBlock,
    //         protocol_swap_fee_percentage,
    //         change.userData
    //     );
    //     ensure_input_length_match_second(balances.len(), amountsInOrOut.len(), dueProtocolFeeAmounts.len());
    //     // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
    //     // their participation.
    //     if let PoolBalanceChangeKind::JOIN = kind {
    //         let finalBalances = _process_join_pool_transfers(sender, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
    //         return (finalBalances, amountsInOrOut, dueProtocolFeeAmounts);
    //     }
    //     else {
    //         let finalBalances = _process_exit_pool_transfers(recipient, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
    //         return (finalBalances, amountsInOrOut, dueProtocolFeeAmounts);
    //     }   
    // }

    // todo dummy value for now
    let v1: Vec<b256> = ~Vec::new();
    let v2: Vec<u64> = ~Vec::new();
    let v3: Vec<u64> = ~Vec::new();
    (v1, v2, v3)
}

// Transfers `amountsIn` from `sender`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees.
// Returns the Pool's final balances, which are the current balances plus `amountsIn` minus accumulated protocol
// swap fees.
#[storage(read, write)]
fn _process_join_pool_transfers(
    sender: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amountsIn: Vec<u64>,
    dueProtocolFeeAmounts: Vec<u64>
) -> Vec<b256> {
    // We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
    let mut wrappedEth = 0;

    let mut finalBalances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amountIn = amountsIn.get(count).unwrap();
        require(amountIn <= change.limits.get(count).unwrap(), "JOIN_ABOVE_MAX");

        // Receive assets from the sender - possibly from Internal Balance.
        let asset: ContractId = change.assets.get(count).unwrap();
        // let x = abi(UserBalance, user_balance_contract_id);
        // x.receive_asset(asset, amountIn, sender, change.useInternalBalance);

        if (_is_eth(asset)) {
            wrappedEth = wrappedEth + amountIn;
        }

        let feeAmount = dueProtocolFeeAmounts.get(count).unwrap();
        // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        // x.pay_fee_amount(_translate_to_ierc20(asset), feeAmount);

        // Compute the new Pool balances. Note that the fee amount might be larger than `amountIn`,
        // resulting in an overall decrease of the Pool's balance for a token.
        // This lets us skip checked arithmetic
        if amountIn >= feeAmount {
            finalBalances.push(increase_cash(balances.get(count).unwrap(), amountIn - feeAmount));
        }
        else {
            finalBalances.push(decrease_cash(balances.get(count).unwrap(), feeAmount - amountIn));
        }
        count = count + 1;
    }

    while count < balances.len() { 
        finalBalances.push(ZERO_B256);
        count = count + 1;
    }

    // Handle any used and remaining ETH.
    handle_remaining_eth(wrappedEth);

    return finalBalances;
}

// Transfers `amountsOut` to `recipient`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees from the Pool.
//
// Returns the Pool's final balances, which are the current `balances` minus `amountsOut` and fees paid
// (`dueProtocolFeeAmounts`).
#[storage(read, write)]
fn _process_exit_pool_transfers(
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amountsOut:Vec<u64>,
    dueProtocolFeeAmounts: Vec<u64>
) -> Vec<b256> {
    let mut finalBalances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amountOut = amountsOut.get(count).unwrap();
        require(amountOut >= change.limits.get(count).unwrap(), "EXIT_BELOW_MIN");

        // Send tokens to the recipient - possibly to Internal Balance
        let asset = change.assets.get(count).unwrap();
        // let x = abi(UserBalance, user_balance_contract_id);
        // x.send_asset(asset, amountOut, recipient, change.useInternalBalance);

        let feeAmount = dueProtocolFeeAmounts.get(count).unwrap();
        // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        // x.pay_fee_amount(_translate_to_ierc20(asset), feeAmount);

        // Compute the new Pool balances. A Pool's token balance always decreases after an exit (potentially by 0).
        finalBalances.push(decrease_cash(balances.get(count).unwrap(), amountOut + feeAmount));
        count = count + 1;
    }
    while count < balances.len() {
        finalBalances.push(ZERO_B256);
        count = count + 1;
    }
    return finalBalances;
}



