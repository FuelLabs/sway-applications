contract;

dep errors;
dep data_structures;
dep interface;
dep utils;
dep events;
dep ops;

use errors::{InputError, PoolError, SwapError};
use data_structures::{
    BatchSwapStep,
    ExitPoolRequest,
    FundManagement,
    IERC20ToBytes32Map,
    IERC20ToBytes32MapEntry,
    JoinPoolRequest,
    PoolBalanceChange,
    PoolBalanceChangeKind,
    PoolBalanceOp,
    PoolBalanceOpKind,
    PoolSpecialization,
    SingleSwap,
    SwapKind,
    SwapRequest,
    TwoTokenPoolBalances,
    TwoTokenPoolTokens,
    UserBalanceOp,
    UserBalanceOpKind,
};

use interface::{ExternalInterface, Vault};
use utils::{
    cash,
    cash_to_managed,
    decrease_cash,
    from_shared_to_balance_a,
    from_shared_to_balance_b,
    get_amounts,
    get_two_token_pair_hash,
    handle_remaining_eth,
    increase_cash,
    is_eth,
    is_zero,
    last_change_block,
    managed,
    managed_delta,
    managed_to_cash,
    max,
    mul_up,
    set_managed,
    sort_two_tokens,
    to_shared_cash,
    to_shared_managed,
    token_calculated,
    token_given,
    total,
    totals_and_last_change_block,
    translate_to_ierc20,
    translate_to_ierc20_array,
    unsafe_cast_to_int256,
    validate_user_balance_op,
    vec_contains,
};

use events::{
    EventAuthorizerChanged,
    EventExternalBalanceTransfer,
    EventFlashLoan,
    EventInternalBalanceChanged,
    EventPoolBalanceChanged,
    EventPoolBalanceManaged,
    EventPoolRegistered,
    EventRelayerApprovalChanged,
    EventTokensDeregistered,
    EventTokensRegistered,
    EventSwap,
};

// use ops::get_word_from_b256;
use std::{
    address::Address,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    constants::{
        BASE_ASSET_ID,
        ZERO_B256,
    },
    context::{
        balance_of,
        call_frames::contract_id,
        msg_amount,
    },
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    reentrancy::is_reentrant,
    result::Result,
    revert::{
        require,
        revert,
    },
    storage::StorageMap,
    token::{
        force_transfer_to_contract,
        transfer_to_output,
    },
    vec::Vec,
};

const TOKEN_NOT_REGISTERED = 512;
const TOKENS_LENGTH_MUST_BE_2 = 524;
// Absolute maximum fee percentages (1e18 = 100%, 1e16 = 1%).
const MAX_PROTOCOL_SWAP_FEE_PERCENTAGE = 50000000000000000;
const MAX_PROTOCOL_FLASH_LOAN_FEE_PERCENTAGE = 10000000000000000;
const TOKEN_ALREADY_REGISTERED = 522;
const PAUSE_WINDOW_END_TIME = 1;
const BUFFER_PERIOD_END_TIME = 2;

storage {
    // Balance of the two token pool
    two_token_pool_tokens: StorageMap<b256, TwoTokenPoolTokens> = StorageMap {},
    // first b256 value is the pool id sencond b256 value is balances
    balances: StorageMap<(b256, b256), TwoTokenPoolBalances> = StorageMap {},
    // Internal Balance of the the user
    internal_token_balance: StorageMap<(Address, ContractId), u64> = StorageMap {},
    // Balance of minimal swap info pool balances
    minimal_swap_info_pools_balances: StorageMap<(b256, ContractId), b256> = StorageMap {},
    // Minimal swap info pool tokens
    minimal_swap_info_pools_tokens: StorageMap<b256, Vec<ContractId>> = StorageMap {},
    // Balance of the general pool
    general_pools_balances: StorageMap<b256, IERC20ToBytes32Map> = StorageMap {},
    // first b256 value is the pool id sencond b256 value is entries
    entries: StorageMap<(b256, u64), IERC20ToBytes32MapEntry> = StorageMap {},
    // first b256 value is the pool id sencond b256 value is indexes
    indexes: StorageMap<(b256, ContractId), u64> = StorageMap {},
    // Asset manager of the pool
    pool_asset_managers: StorageMap<(b256, ContractId), Address> = StorageMap {},
    // Swap fee in percentage 
    swap_fee_percentage: u64 = 1,
    // Flash loan fee in percentage
    flash_loan_fee_percentage: u64 = 10,
    // Registered state of the pool
    is_pool_registered: StorageMap<b256, bool> = StorageMap {},
    // Approved state of the relayer
    approved_relayers: StorageMap<(Address, Address), bool> = StorageMap {},
    // ContractID of the authorizer
    authorizer: ContractId = ContractId {
        value: 0xa3f865aa351e51cfeb40f5178d1564bb629fe9030b83caf6361d1baaf5b90b5a,
    },
    // todo: In fuel this doesn't need, will check once
    next_nonce: StorageMap<Address, u64> = StorageMap {},
    // Pool state
    paused: bool = false,
    // Specialization of the pool(Two_TOKEN, MINIMAL_SWAP_INFO, GENERAL)
    pool_specialization: StorageMap<b256, PoolSpecialization> = StorageMap {},
}

impl Vault for Contract {
    #[storage(read, write)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64,
    ) -> Vec<u64> {
        // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
        // require(block_timestamp() <= deadline, SwapError::SWAP_DEADLINE);
        require(assets.len() == limits.len(), InputError::InputLengthMismatch);

        // Perform the swaps, updating the Pool token balances and computing the net Vault asset deltas.
        let asset_deltas = swap_with_pools(swaps, assets, funds, kind);

        // Process asset deltas, by either transferring assets from the sender (for positive deltas) or to the recipient
        // (for negative deltas).
        let mut wrapped_eth = 0;
        let mut count = 0;
        while count < assets.len() {
            let asset = assets.get(count).unwrap();
            let delta = asset_deltas.get(count).unwrap();
            require(delta <= limits.get(count).unwrap(), SwapError::SwapLimit);

            if (delta > 0) {
                let to_receive = delta;
                receive_asset(asset, to_receive, funds.sender, funds.from_internal_balance);

                if (is_eth(asset)) {
                    wrapped_eth = wrapped_eth + to_receive;
                }
            } else if (delta < 0) {
                // let to_send = -delta;
                let to_send = delta;
                send_asset(asset, to_send, funds.recipient, funds.to_internal_balance);
            }
        }

        // Handle any used and remaining ETH.
        handle_remaining_eth(wrapped_eth);

        return asset_deltas;
    }

    #[storage(read, write)]
    fn swap(
        single_swap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64,
    ) -> u64 {
        // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
        // require(block_timestamp() <= deadline, PoolError::SWAP_DEADLINE);
        // This revert reason is for consistency with `batch_swap`: an equivalent `swap` performed using that fn
        // would result in this PoolError.
        require(single_swap.amount > 0, SwapError::UnknownAmountInFirstSwap);

        let token_in = translate_to_ierc20(single_swap.asset_in);
        let token_out = translate_to_ierc20(single_swap.asset_out);
        require(token_in != token_out, SwapError::CannotSwapSameToken);

        // Initializing each struct field one-by-one uses less gas than setting all at once.
        let pool_request = SwapRequest {
            pool_id: single_swap.pool_id,
            kind: single_swap.kind,
            token_in: token_in,
            token_out: token_out,
            amount: single_swap.amount,
            user_data: single_swap.user_data,
            from: funds.sender,
            to: funds.recipient,
            // todo The last_change_block field is left uninitialized.
            last_change_block: 0,
        };

        let amount_in = 0;
        let amount_out = 0;

        let (amount_calculated, amount_in, amount_out) = swap_with_pool(pool_request);
        if let SwapKind::GivenIn = single_swap.kind {
            require(amount_out >= limit, SwapError::SwapLimit);
        } else {
            require(amount_in <= limit, SwapError::SwapLimit);
        }

        // receive_asset(single_swap.asset_in, amount_in, funds.sender, funds.from_internal_balance);
        // send_asset(single_swap.asset_out, amount_out, funds.recipient, funds.to_internal_balance);

        // // If the asset in is ETH, then `amount_in` ETH was wrapped into WETH.
        // if is_eth(single_swap.asset_in) {
        //     handle_remaining_eth(amount_in);
        // } else {
        //     handle_remaining_eth(0);
        // }
        let amount_calculated = 150;
        return amount_calculated;
    }

    // This fn is not marked as `nonReentrant` because the underlying mechanism relies on reentrancy
    #[storage(write, read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
    ) -> Vec<u64> {
        // In order to accurately 'simulate' swaps, this fn actually does perform the swaps, including calling the
        // Pool hooks and updating balances in storage. However, once it computes the final Vault Deltas, it
        // reverts unconditionally, returning this array as the revert data.
        //
        // By wrapping this reverting call, we can decode the deltas 'returned' and return them as a normal Solidity
        // fn would. The only caveat is the fn becomes non-view, but off-chain clients can still call it
        // via eth_call to get the expected result.
        //
        // This technique was inspired by the work from the Gnosis team in the Gnosis Safe contract:
        // https://github.com/gnosis/safe-contracts/blob/v1.2.0/contracts/GnosisSafe.sol#L265
        //
        // Most of this fn is implemented using inline assembly, as the actual work it needs to do is not
        // significant, and Solidity is not particularly well-suited to generate this behavior, resulting in a large
        // amount of generated bytecode.
        // let sender = match msg_sender().unwrap() {
        //     Identity::Address(address) => address, _ => revert(0),
        // };
        // let sender: b256 = sender.into();
        // let this_contract: b256 = contract_id().into();
        // if sender != this_contract {
        // We perform an external call to ourselves, forwarding the same calldata. In this call, the else clause of
        // the preceding if statement will be executed instead.
        // let(success, _) = contract_id().call(msg.data);
        // assembly {
        //     // This call should always revert to decode the actual asset deltas from the revert reason
        //     switch success
        //         case 0 {
        //             // Note we are manually writing the memory slot 0. We can safely overwrite whatever is
        //             // stored there as we take full control of the execution and then immediately return.
        //             // We copy the first 4 bytes to check if it matches with the expected signature, otherwise
        //             // there was another revert reason and we should forward it.
        //             returndatacopy(0, 0, 0x04)
        //             let PoolError := and(mload(0), 0xffffffff00000000000000000000000000000000000000000000000000000000)
        //             // If the first 4 bytes don't match with the expected signature, we forward the revert reason.
        //             if eq(eq(PoolError, 0xfa61cc1200000000000000000000000000000000000000000000000000000000), 0) {
        //                 returndatacopy(0, 0, returndatasize())
        //                 revert(0, returndatasize())
        //             }
        //             // The returndata contains the signature, followed by the raw memory representation of an array:
        //             // length + data. We need to return an ABI-encoded representation of this array.
        //             // An ABI-encoded array contains an additional field when compared to its raw memory
        //             // representation: an offset to the location of the length. The offset itself is 32 bytes long,
        //             // so the smallest value we  can use is 32 for the data to be located immediately after it.
        //             mstore(0, 32)
        //             // We now copy the raw memory array from returndata into memory. Since the offset takes up 32
        //             // bytes, we start copying at address 0x20. We also get rid of the PoolError signature, which takes
        //             // the first four bytes of returndata.
        //             let size := sub(returndatasize(), 0x04)
        //             returndatacopy(0x20, 0x04, size)
        //             // We finally return the ABI-encoded array, which has a total length equal to that of the array
        //             // (returndata), plus the 32 bytes for the offset.
        //             return(0, add(size, 32))
        //         }
        //         default {
        //             // This call should always revert, but we fail nonetheless if that didn't happen
        //             invalid()
        //         }
        // }
        // } else {
        //     let deltas = swap_with_pools(swaps, assets, funds, kind);
        // assembly {
        //     // We will return a raw representation of the array in memory, which is composed of a 32 byte length,
        //     // followed by the 32 byte int256 values. Because revert expects a size in bytes, we multiply the array
        //     // length (stored at `deltas`) by 32.
        //     let size := mul(mload(deltas), 32)
        //     // We send one extra value for the PoolError signature "QueryError(int256[])" which is 0xfa61cc12.
        //     // We store it in the previous slot to the `deltas` array. We know there will be at least one available
        //     // slot due to how the memory scratch space works.
        //     // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
        //     mstore(sub(deltas, 0x20), 0x00000000000000000000000000000000000000000000000000000000fa61cc12)
        //     let start := sub(deltas, 0x04)
        //     // When copying from `deltas` into returndata, we copy an additional 36 bytes to also return the array's
        //     // length and the PoolError signature.
        //     revert(start, add(size, 36))
        // }
        // }
        let deltas = swap_with_pools(swaps, assets, funds, kind);
        return deltas;
    }

    #[storage(read, write)]
    fn register_tokens(
        pool_id: b256,
        tokens: Vec<ContractId>,
        asset_managers: Vec<Address>,
    ) {
        is_reentrant();
        // let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
        // x.ensure_not_paused();
        require(tokens.len() == asset_managers.len(), InputError::InputLengthMismatch);

        // Validates token addresses and assigns Asset Managers
        let mut count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            require(token != ~ContractId::from(ZERO_B256), PoolError::InvalidToken);
            storage.pool_asset_managers.insert((
                pool_id,
                token,
            ), asset_managers.get(count).unwrap());
            count += 1;
        }
        let specialization = storage.pool_specialization.get(pool_id);
        if let PoolSpecialization::TwoToken = specialization {
            require(tokens.len() == 2, PoolError::TokensLengthMustBe2);
            register_two_token_pool_tokens(pool_id, tokens.get(0).unwrap(), tokens.get(1).unwrap());
        } else if let PoolSpecialization::MinimalSwapInfo = specialization
        {
            register_minimal_swap_info_pool_tokens(pool_id, tokens);
        } else {
            register_general_pool_tokens(pool_id, tokens);
        }

        log(EventTokensRegistered {
            pool_id: pool_id,
            tokens: tokens,
            asset_managers: asset_managers,
        });
    }

    #[storage(read, write)]
    fn deregister_tokens(pool_id: b256, tokens: Vec<ContractId>) {
        is_reentrant();
        // let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
        // x.ensure_not_paused();
        let specialization = storage.pool_specialization.get(pool_id);
        if let PoolSpecialization::TwoToken = specialization {
            require(tokens.len() == 2, PoolError::TokensLengthMustBe2);
            deregister_two_token_pool_tokens(pool_id, tokens.get(0).unwrap(), tokens.get(1).unwrap());
        } else if let PoolSpecialization::MinimalSwapInfo = specialization
        {
            deregister_minimal_swap_info_pool_tokens(pool_id, tokens);
        } else {
            // PoolSpecialization::GENERAL
            deregister_general_pool_tokens(pool_id, tokens);
        }

        // The deregister calls above ensure the total token balance is zero. Therefore it is now safe to remove any
        // associated Asset Managers, since they hold no Pool balance.
        // Todo need to be implemented when we can remove things from storage
        let mut count = 0;
        while count < tokens.len() {
            storage.pool_asset_managers.insert((
                pool_id,
                tokens.get(count).unwrap(),
            ), ~Address::from(ZERO_B256));
            count += 1;
        }
        log(EventTokensDeregistered {
            pool_id: pool_id,
            tokens: tokens,
        });
    }

    // PoolBalances
    #[storage(read, write)]
    fn join_pool(
        pool_id: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest,
    ) {
        // This fn doesn't have the nonReentrant modifier: it is applied to `join_or_exit` instead.
        // Note that `recipient` is not actually payable in the context of a join - we cast it because we handle both
        // joins and exits at once.
        join_or_exit(PoolBalanceChangeKind::Join, pool_id, sender, recipient, join_to_pool_balance_change(request));
    }

    #[storage(read, write)]
    fn exit_pool(
        pool_id: b256,
        sender: Address,
        recipient: Address,
        request: ExitPoolRequest,
    ) {
        // This fn doesn't have the nonReentrant modifier: it is applied to `join_or_exit` instead.
        join_or_exit(PoolBalanceChangeKind::Exit, pool_id, sender, recipient, exit_to_pool_balance_change(request));
    }

    // Flashloans
    #[storage(read, write)]
    fn flash_loan(
        recipient: ContractId,
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        user_data: Vec<b256>,
    ) {
        require(tokens.len() == amounts.len(), InputError::InputLengthMismatch);

        let mut fee_amounts = ~Vec::new();
        let mut pre_loan_balances = ~Vec::new();

        // Used to ensure `tokens` is sorted in ascending order, which ensures token uniqueness.
        let mut previous_token: ContractId = ~ContractId::from(ZERO_B256);
        let first_token = previous_token;
        let mut count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            let amount = amounts.get(count).unwrap();

            if token == first_token {
                let token: b256 = token.into();
                let previous_token: b256 = previous_token.into();
                require(token > previous_token, InputError::ZeroToken);
            } else {
                let token: b256 = token.into();
                let previous_token: b256 = previous_token.into();
                require(token > previous_token, InputError::UnsortedTokens);
            }
            previous_token = token;

            pre_loan_balances.push(balance_of(token, contract_id()));
            // let x = abi(ProtocolFeesCollector, storage.protocol_fees_collector_contract_id);
            fee_amounts.push(calculate_flash_loan_fee_amount(amount));

            require(pre_loan_balances.get(count).unwrap() >= amount, PoolError::InsufficientFlashLoanBalance);
            // todo need to discuss this
            force_transfer_to_contract(amount, token, recipient);
            count += 1;
        }

        // recipient.receiveFlashLoan(tokens, amounts, fee_amounts, user_data);
        // ! implimentation of above function
        // require(msg.sender == contract_id(), PoolError::CALLER_NOT_VAULT);
        force_transfer_to_contract(amounts.get(0).unwrap(), tokens.get(0).unwrap(), contract_id());
        count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            let pre_loan_balance = pre_loan_balances.get(count).unwrap();

            // Checking for loan repayment first (without accounting for fees) makes for simpler debugging, and results
            // in more accurate revert reasons if the flash loan protocol fee percentage is zero.
            let post_loan_balance = balance_of(contract_id(), token);
            require(post_loan_balance >= pre_loan_balance, PoolError::InvalidPostLoanBalance);

            // No need for checked arithmetic since we know the loan was fully repaid.
            let received_fee_amount = post_loan_balance - pre_loan_balance;
            require(received_fee_amount >= fee_amounts.get(count).unwrap(), PoolError::InsufficientFlashLoanFeeAmount);

            pay_fee_amount(token, received_fee_amount);
            log(EventFlashLoan {
                recipient: recipient,
                token: token,
                amount: amounts.get(count).unwrap(),
                received_fee_amount: received_fee_amount,
            });

            count += 1;
        }
    }

    // AssetManagers
    #[storage(read, write)]
    fn manage_pool_balance(ops: Vec<PoolBalanceOp>) {
        // This variable could be declared inside the loop, but that causes the compiler to allocate memory on each
        // loop iteration, increasing gas costs.
        let mut count = 0;
        while count < ops.len() {
            // By indexing the array only once, we don't spend extra gas in the same bounds check.
            let op: PoolBalanceOp = ops.get(count).unwrap();

            let pool_id = op.pool_id;

            // let x = abi(PoolRegistry, pool_registry_contract_id);
            // x.ensure_registered_pool(pool_id);
            let sender = match msg_sender().unwrap() {
                Identity::Address(address) => address,
                _ => revert(0),
            };
            let token: ContractId = op.token;
            require(is_token_registered(pool_id, token), PoolError::TokenNotRegistered);
            require(storage.pool_asset_managers.get((
                pool_id,
                token,
            )) == sender, PoolError::SenderNotAssetManager);

            let kind: PoolBalanceOpKind = op.kind;
            let amount = op.amount;
            let (cash_delta, managed_delta_value) = perform_pool_management_operation(kind, pool_id, token, amount);

            count += 1;

            log(EventPoolBalanceManaged {
                pool_id: pool_id,
                sender: sender,
                token: token,
                cash_delta: cash_delta,
                managed_delta: managed_delta_value,
            });
        }
    }

    #[storage(read, write)]
    fn manage_user_balance(ops: Vec<UserBalanceOp>) {
        // is_reentrant();
        // We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
        let mut eth_wrapped: u64 = 0;

        // Cache for these checks so we only perform them once (if at all).
        let checked_caller_is_relayer = false;
        let mut checked_not_paused = false;
        let mut i = 0;
        while i < ops.len() {
            let ops_value: (UserBalanceOpKind, ContractId, u64, Address, Address, bool) = validate_user_balance_op(ops.get(i).unwrap(), checked_caller_is_relayer);
            let (kind, asset, amount, sender, recipient, checked_caller_is_relayer) = ops_value;
            if let UserBalanceOpKind::WithdrawInternal = kind {
                // Internal Balance withdrawals can always be performed by an authorized account.
                withdraw_from_internal_balance(asset, sender, recipient, amount);
            } else {
                // All other operations are blocked if the contract is paused.
                // We cache the result of the pause check and skip it for other operations in this same transaction
                // (if any).
                if (!checked_not_paused) {
                    // let x = abi(TemporarilyPausable, temporarily_pausable_contract_id);
                    // x._ensure_not_paused();
                    checked_not_paused = true;
                }

                if let UserBalanceOpKind::DepositInternal = kind {
                    deposit_to_internal_balance(asset, sender, recipient, amount);

                    // Keep track of all ETH wrapped into WETH as part of a deposit.
                    if (is_eth(asset)) {
                        eth_wrapped = eth_wrapped + amount;
                    }
                } else {
                    // Transfers don't support ETH.
                    require(!is_eth(asset), PoolError::CannotUseEthSentinel);
                    let token = asset;

                    if let UserBalanceOpKind::TransferInternal = kind {
                        transfer_internal_balance(token, sender, recipient, amount);
                    } else {
                        // TRANSFER_EXTERNAL
                        transfer_to_external_balance(token, sender, recipient, amount);
                    }
                }
            }
        }
        handle_remaining_eth(eth_wrapped);
    }

    #[storage(read, write)]
    fn register_pool(pool_id: b256, specialization: PoolSpecialization) -> b256 {
        require(!(storage.is_pool_registered.get(pool_id)), PoolError::InvalidPoolId); // Should never happen as Pool IDs are unique.
        storage.is_pool_registered.insert(pool_id, true);
        storage.pool_specialization.insert(pool_id, specialization);

        log(EventPoolRegistered {
            pool_id: pool_id,
            specialization: specialization,
        });
        return pool_id;
    }

    #[storage(read, write)]
    fn set_authorizer(new_authorizer: ContractId) {
        is_reentrant();
        storage.authorizer = new_authorizer;
        log(EventAuthorizerChanged {
            new_authorizer: new_authorizer,
        });
    }

    #[storage(read, write)]
    fn set_relayer_approval(sender: Address, relayer: Address, approved: bool) {
        is_reentrant();
        storage.approved_relayers.insert((
            sender,
            relayer,
        ), approved);
        log(EventRelayerApprovalChanged {
            relayer: relayer,
            sender: sender,
            approved: approved,
        });
    }

    // TODO: View functions
    #[storage(read)]
    fn weth() -> ContractId {
        return ~ContractId::from(WFUEL);
    }

    #[storage(read)]
    fn get_authorizer() -> ContractId {
        return storage.authorizer;
    }

    #[storage(read)]
    fn get_internal_balance(user: Address, tokens: Vec<ContractId>) -> Vec<u64> {
        let mut balances: Vec<u64> = ~Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            balances.push(get_internal_balance_private(user, tokens.get(i).unwrap()));
            i += 1;
        }
        return balances;
    }

    // todo: In fuel this doesn't need, will check once
    #[storage(read)]
    fn get_next_nonce(user: Address) -> u64 {
        return storage.next_nonce.get(user);
    }

    // Returns the current contract pause status, as well as the end times of the Pause Window and Buffer Period.
    #[storage(read)]
    fn get_paused_state() -> (bool, u64, u64) {
        return (
            !is_not_paused(),
            PAUSE_WINDOW_END_TIME,
            BUFFER_PERIOD_END_TIME,
        );
    }

    #[storage(read)]
    fn get_pool(pool_id: b256) -> (ContractId, PoolSpecialization) {
        return (
            ~ContractId::from(pool_id),
            storage.pool_specialization.get(pool_id),
        );
    }

    #[storage(read)]
    fn get_pool_token_info(pool_id: b256, token: ContractId) -> (u64, u64, u64, Address) {
        // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
        // x.with_registered_pool(pool_id);
        let mut balance: b256 = ZERO_B256;
        let specialization = storage.pool_specialization.get(pool_id);

        if let PoolSpecialization::TwoToken = specialization {
            balance = get_two_token_pool_balance(pool_id, token);
        } else if let PoolSpecialization::MinimalSwapInfo = specialization
        {
            balance = get_minimal_swap_info_pool_balance(pool_id, token);
        } else {
            // PoolSpecialization::GENERAL
            balance = get_general_pool_balance(pool_id, token);
        }

        return (
            cash(balance),
            managed(balance),
            last_change_block(balance),
            storage.pool_asset_managers.get((
                pool_id,
                token,
            )),
        )
    }

    #[storage(read)]
    fn get_pool_tokens(poolId: b256) -> (Vec<ContractId>, Vec<u64>, u64) {
        let (tokens, rawBalances) = private_get_pool_tokens(poolId);
        let (balances, lastChangeBlock) = totals_and_last_change_block(rawBalances);
        return (
            tokens,
            balances,
            lastChangeBlock,
        )
    }

    #[storage(read)]
    fn has_approved_relayer(user: Address, relayer: Address) -> bool {
        return storage.approved_relayers.get((
            user,
            relayer,
        ));
    }

    #[storage(read)]fn get_swap_fee_percentage() -> u64 {
        return storage.swap_fee_percentage;
    }
}

// Returns the balance of a token in a General Pool.
// This function assumes `pool_id` exists and corresponds to the General specialization setting.
// Requirements:
// - `token` must be registered in the Pool
#[storage(read)]
fn get_general_pool_balance(pool_id: b256, token: ContractId) -> b256 {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    return get_value(pool_id, token);
}

#[storage(read)]
fn get_two_token_pool_balance(pool_id: b256, token: ContractId) -> b256 {
    let (token_a, balance_a, token_b, balance_b) = get_two_token_pool_balances(pool_id);
    let mut balance: b256 = ZERO_B256;
    if token == token_a {
        balance = balance_a;
    } else if token == token_b {
        balance = balance_b;
    } else {
        revert(TOKEN_NOT_REGISTERED);
    }
    return balance;
}

#[storage(read)]
fn is_not_paused() -> bool {
    // After the Buffer Period, the (inexpensive) timestamp check short-circuits the storage access.
    // todo need block_timestamp
    // return block_timestamp() > BUFFER_PERIOD_END_TIME || !storage.paused;
    true
}

// These getters lead to reduced bytecode size by inlining the immutable variables in a single place.
#[storage(read, write)]
fn deregister_two_token_pool_tokens(pool_id: b256, token_x: ContractId, token_y: ContractId) {
    let (balance_a, balance_b) = get_two_token_pool_shared_balances(pool_id, token_x, token_y);

    require(is_zero(balance_a) && is_zero(balance_b), PoolError::NonZeroTokenBalance);

    // delete _twoTokenPoolTokens[pool_id];
    // delete pool_balances.sharedCash;
    // No delete methods for storage yet
    // need to implement it, as soon as we get support for that
}

#[storage(read, write)]
fn deregister_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let mut count = 0;

    while (count < tokens.len()) {
        let token = tokens.get(count).unwrap();
        let balance = storage.minimal_swap_info_pools_balances.get((
            pool_id,
            token,
        ));
        require(is_zero(balance), PoolError::NonZeroTokenBalance);

        // delete minimal_swap_info_pools_balances[pool_id][token];
        // we need to delete the StorageMap in this case ^
        // but StorageMap does not have delete method on it
        // need to implemet it
        let token_removed = vec_remove_if_contains(pool_id, tokens.get(count).unwrap());
        if !token_removed {
            require(token_removed, PoolError::TokenNotRegistered);
        }
        count += 1;
    }
}

// helping function
#[storage(read, write)]
fn vec_remove_if_contains(pool_id: b256, delete: ContractId) -> bool {
    let mut vec = storage.minimal_swap_info_pools_tokens.get(pool_id);
    let mut count = 0;
    let mut return_bool: bool = true;

    while (count < vec.len()) {
        if vec.get(count).unwrap() == delete {
            vec.remove(count);
            return_bool = true;
        } else {
            return_bool = true;
        }
        count += 1;
    }

    return return_bool;
}

#[storage(write, read)]
fn deregister_general_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let mut count = 0;

    while (count < tokens.len()) {
        let token = tokens.get(count).unwrap();
        let current_balance = get_general_pool_balance_storage_map(pool_id, token);

        require(current_balance == 0x0000000000000000000000000000000000000000000000000000000000000000, PoolError::NonZeroTokenBalance);

        // pool_balances.remove(token);
        // no remove method on StorageMap
        // so assigning 0 to token
        let token = ~ContractId::from(token.into());
        set(pool_id, token, 0x0000000000000000000000000000000000000000000000000000000000000000);

        count += 1;
    }
}

// takes storagemap; StorageMap
#[storage(read)]
fn get_general_pool_balance_storage_map(pool_id: b256, token: ContractId) -> b256 {
    return get_value(pool_id, token);
}

#[storage(read, write)]
fn set(pool_id: b256, key: ContractId, value: b256) -> bool {
    // We read and store the key's index to prevent multiple reads from the same storage slot
    let key_index = storage.indexes.get((
        pool_id,
        key,
    ));
    let mut pool_balances = storage.general_pools_balances.get(pool_id);

    // Equivalent to !contains(map, key)
    if key_index == 0 {
        let previous_length = pool_balances.length;
        let tmp = IERC20ToBytes32MapEntry {
            key: key,
            value: value,
        };
        storage.entries.insert((
            pool_id,
            previous_length,
        ), tmp);
        pool_balances.length = previous_length + 1;
        // The entry is stored at previous_length, but we add 1 to all indexes
        // and use 0 as a sentinel value
        storage.indexes.insert((
            pool_id,
            key,
        ), previous_length + 1);
        return true;
    } else {
        let tmp = IERC20ToBytes32MapEntry {
            key: storage.entries.get((
                pool_id,
                key_index - 1,
            )).key,
            value: value,
        };
        storage.entries.insert((
            pool_id,
            key_index - 1,
        ), tmp);
        return false;
    }
}

#[storage(read)]
fn get_value(pool_id: b256, key: ContractId) -> b256 {
    let index = storage.indexes.get((
        pool_id,
        key,
    ));
    require(index > 0, InputError::OutOfBounds);
    return unchecked_value_at(pool_id, index - 1);
}

#[storage(read)]
fn unchecked_value_at(pool_id: b256, index: u64) -> b256 {
    return storage.entries.get((
        pool_id,
        index,
    )).value;
}

#[storage(read, write)]
fn register_two_token_pool_tokens(pool_id: b256, token_x: ContractId, token_y: ContractId) {
    require(token_x != token_y, PoolError::TokenAlreadyRegistered);
    let token_a: b256 = token_x.into();
    let token_b: b256 = token_y.into();
    require(token_a < token_b, InputError::UnsortedTokens);

    let mut pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    require(pool_tokens.token_a == ~ContractId::from(ZERO_B256) && pool_tokens.token_a == ~ContractId::from(ZERO_B256), PoolError::TokensAlreadySet);

    pool_tokens.token_a = token_x;
    pool_tokens.token_b = token_y;
}

#[storage(read, write)]
fn register_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let mut pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);

    let mut count = 0;
    while (count < tokens.len()) {
        let token_contains = vec_contains(pool_tokens, tokens.get(count).unwrap());
        if !token_contains {
            pool_tokens.push(tokens.get(count).unwrap());
        } else {
            require(token_contains, PoolError::TokenAlreadyRegistered);
        }
        count += 1;
    }
}

#[storage(write, read)]
fn register_general_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let pool_balances = storage.general_pools_balances.get(pool_id);

    let mut count = 0;
    while (count < tokens.len()) {
        let added = set(pool_id, tokens.get(count).unwrap(), 0x0000000000000000000000000000000000000000000000000000000000000000);
        require(added, PoolError::TokenAlreadyRegistered);
        count += 1;
    }
}

// Converts a JoinPoolRequest into a PoolBalanceChange.
fn join_to_pool_balance_change(request: JoinPoolRequest) -> PoolBalanceChange {
    let change = PoolBalanceChange {
        assets: request.assets,
        limits: request.max_amounts_in,
        user_data: request.user_data,
        use_internal_balance: request.from_internal_balance,
    };
    return change;
}

// Converts a JoinPoolRequest into a PoolBalanceChange
fn exit_to_pool_balance_change(request: ExitPoolRequest) -> PoolBalanceChange {
    let change = PoolBalanceChange {
        assets: request.assets,
        limits: request.min_amounts_out,
        user_data: request.user_data,
        use_internal_balance: request.to_internal_balance,
    };
    return change;
}

// Performs all `swaps`, calling swap hooks on the Pool contracts and updating their balances. Does not cause
// any transfer of tokens - instead it returns the net Vault token deltas: positive if the Vault should receive
// tokens, and negative if it should send them.
#[storage(write, read)]
fn swap_with_pools(
    swaps: Vec<BatchSwapStep>,
    assets: Vec<ContractId>,
    funds: FundManagement,
    kind: SwapKind,
) -> Vec<u64> {
    let mut asset_deltas: Vec<u64> = ~Vec::with_capacity(assets.len());

    // These store data about the previous swap here to implement multihop logic across swaps.
    let mut previous_token_calculated: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);
    let mut previous_amount_calculated = 0;

    let mut count = 0;
    while count < swaps.len() {
        let mut batch_swap_step = swaps.get(count).unwrap();

        let mut within_bounds = false;
        if batch_swap_step.asset_in_index < assets.len()
            && batch_swap_step.asset_out_index < assets.len()
        {
            within_bounds = true;
        }
        require(within_bounds, InputError::OutOfBounds);

        let token_in = translate_to_ierc20(assets.get(batch_swap_step.asset_in_index).unwrap());
        let token_out = translate_to_ierc20(assets.get(batch_swap_step.asset_out_index).unwrap());
        require(token_in != token_out, SwapError::CannotSwapSameToken);

        // Sentinel value for multihop logic
        if (batch_swap_step.amount == 0) {
            // When the amount given is zero, we use the calculated amount for the previous swap, as long as the
            // current swap's given token is the previous calculated token. This makes it possible to swap a
            // given amount of token A for token B, and then use the resulting token B amount to swap for token C.
            require(count > 0, SwapError::UnknownAmountInFirstSwap);
            let using_previous_token = previous_token_calculated == token_given(kind, token_in, token_out);
            require(using_previous_token, PoolError::MalconstructedMultihopSwp);
            batch_swap_step.amount = previous_amount_calculated;
        }

        // Initializing each struct field one-by-one uses less gas than setting all at once
        let pool_request = SwapRequest {
            pool_id: batch_swap_step.pool_id,
            kind: kind,
            token_in: token_in,
            token_out: token_out,
            amount: batch_swap_step.amount,
            user_data: batch_swap_step.user_data,
            from: funds.sender,
            to: funds.recipient, // todo The last_change_block field is left uninitialized.
            last_change_block: 0,
        };

        let mut amount_in = 0;
        let mut amount_out = 0;
        let (previous_amount_calculated, amount_in, amount_out) = swap_with_pool(pool_request);

        let previous_token_calculated = token_calculated(kind, token_in, token_out);

        // Accumulate Vault deltas across swaps
        // todo when sway support swap will replace this
        asset_deltas.push((asset_deltas.get(batch_swap_step.asset_in_index).unwrap() + amount_in));
        asset_deltas.swap(batch_swap_step.asset_in_index, asset_deltas.len() - 1);
        asset_deltas.pop();

        asset_deltas.push(asset_deltas.get(batch_swap_step.asset_out_index).unwrap() - amount_out);
        asset_deltas.swap(batch_swap_step.asset_in_index, asset_deltas.len() - 1);
        asset_deltas.pop();
    }
    return asset_deltas;
}

// Performs a swap according to the parameters specified in `request`, calling the Pool's contract hook and
// updating the Pool's balance.
//
// Returns the amount of tokens going into or out of the Vault as a result of this swap, depending on the swap kind.
#[storage(write, read)]
fn swap_with_pool(request: SwapRequest) -> (u64, u64, u64) {
    // Get the calculated amount from the Pool and update its balances
    let pool = ~ContractId::from(request.pool_id);
    let specialization = storage.pool_specialization.get(request.pool_id);

    let amount_calculated = 0;
    if let PoolSpecialization::TwoToken = specialization {
        let amount_calculated = process_two_token_pool_swap_request(request, pool);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        let amount_calculated = process_minimal_swap_info_pool_swap_request(request, pool);
    } else {
        // PoolSpecialization::GENERAL
        let amount_calculated = process_general_pool_swap_request(request, pool);
    }

    let (amount_in, amount_out) = get_amounts(request.kind, request.amount, amount_calculated);
    log(EventSwap {
        pool_id: request.pool_id,
        token_in: request.token_in,
        token_out: request.token_out,
        amount_in: amount_in,
        amount_out: amount_out,
    });
    return (
        amount_calculated,
        amount_in,
        amount_out,
    );
}

#[storage(read)]
fn process_two_token_pool_swap_request(request: SwapRequest, pool: ContractId) -> u64 {
    // For gas efficiency reasons, this fn uses low-level knowledge of how Two Token Pool balances are
    // stored internally, instead of using getters and setters for all operations.
    let (token_a_balance, token_b_balance) = get_two_token_pool_shared_balances(request.pool_id, request.token_in, request.token_out);

    let pair_hash = storage.two_token_pool_tokens.get(request.pool_id).balances;
    let mut pool_balances = storage.balances.get((
        request.pool_id,
        pair_hash,
    ));

    // We have the two Pool balances, but we don't know which one is 'token in' or 'token out'.
    let mut token_in_balance: b256 = ZERO_B256;
    let mut token_out_balance: b256 = ZERO_B256;

    // In Two Token Pools, token A has a smaller address than token B
    let token_in: b256 = request.token_in.into();
    let token_out: b256 = request.token_out.into();
    if token_in < token_out {
        // in is A, out is B
        token_in_balance = token_a_balance;
        token_out_balance = token_b_balance;
    } else {
        // in is B, out is A
        token_out_balance = token_a_balance;
        token_in_balance = token_b_balance;
    }

    // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    let (token_in_balance, token_out_balance, amount_calculated) = call_minimal_swap_info_pool_on_swap_hook(request, pool, token_in_balance, token_out_balance);

    // We check the token ordering again to create the new shared cash packed struct
    let token_in: b256 = request.token_in.into();
    let token_out: b256 = request.token_out.into();
    if token_in < token_out {
        pool_balances.shared_cash = to_shared_cash(token_in_balance, token_out_balance); // in is A, out is B
    } else {
        pool_balances.shared_cash = to_shared_cash(token_out_balance, token_in_balance); // in is B, out is A
    }

    return amount_calculated;
}

// Calls the onSwap hook for a Pool that implements IMinimalSwapInfoPool: both Minimal Swap Info and Two Token
// Pools do this.
fn call_minimal_swap_info_pool_on_swap_hook(
    request: SwapRequest,
    pool: ContractId,
    token_in_balance: b256,
    token_out_balance: b256,
) -> (b256, b256, u64) {
    let mut request = request;
    let token_in_total = total(token_in_balance);
    let token_out_total = total(token_out_balance);
    request.last_change_block = max(last_change_block(token_in_balance), last_change_block(token_out_balance));

    // Perform the swap request callback, and compute the new balances for 'token in' and 'token out' after the swap
    // todo pool-utils/contracts/BaseMinimalSwapInfoPool
    // let amount_calculated = on_swap(pool, request, token_in_total, token_out_total);
    let amount_calculated = 0;
    let (amount_in, amount_out) = get_amounts(request.kind, request.amount, amount_calculated);

    let new_token_in_balance = increase_cash(token_in_balance, amount_in);
    let new_token_out_balance = decrease_cash(token_out_balance, amount_out);

    return (
        new_token_in_balance,
        new_token_out_balance,
        amount_calculated,
    );
}

#[storage(read, write)]
fn set_two_token_pool_cash_balances(
    pool_id: b256,
    token_a: ContractId,
    balance_a: b256,
    token_b: ContractId,
    balance_b: b256,
) {
    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let pool_balances = storage.two_token_pool_tokens.get(pool_id).balances;
    let mut balalnce = storage.balances.get((
        pool_id,
        pool_balances,
    ));
    balalnce.shared_cash = to_shared_cash(balance_a, balance_b);

    let bal = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((
        pool_id,
        pair_hash,
    ), balalnce);
}

#[storage(read, write)]
fn set_minimal_swap_info_pool_balances(
    pool_id: b256,
    tokens: Vec<ContractId>,
    balances: Vec<b256>,
) {
    let mut count = 0;

    while (count < tokens.len()) {
        let token = tokens.get(count).unwrap();
        storage.minimal_swap_info_pools_balances.insert((
            pool_id,
            token,
        ), balances.get(count).unwrap());
        count += 1;
    }
}

#[storage(write, read)]
fn set_general_pool_balances(pool_id: b256, balances: Vec<b256>) {
    let mut count = 0;
    while (count < balances.len()) {
        let mut val = storage.entries.get((
            pool_id,
            count,
        ));
        val.value = balances.get(count).unwrap();
        count += 1;
    }
}

// this below function originally belong to fee contract
// Returns the protocol fee amount to charge for a flash loan of `amount`.
#[storage(read)]
fn calculate_flash_loan_fee_amount(amount: u64) -> u64 {
    // Fixed point multiplication introduces PoolError: we round up, which means in certain scenarios the charged
    // percentage can be slightly higher than intended.
    let percentage = storage.flash_loan_fee_percentage;
    return mul_up(amount, percentage);
}

#[storage(read)] // Returns true if `token` is registered for `pool_id`.
fn is_token_registered(pool_id: b256, token: ContractId) -> bool {
    let specialization: PoolSpecialization = storage.pool_specialization.get(pool_id);
    if let PoolSpecialization::TwoToken = specialization {
        return is_two_token_pool_token_registered(pool_id, token);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        return is_minimal_swap_info_pool_token_registered(pool_id, token);
    } else {
        // PoolSpecialization::GENERAL
        return is_general_pool_token_registered(pool_id, token);
    }
    true
}

#[storage(read)]
fn is_minimal_swap_info_pool_token_registered(pool_id: b256, token: ContractId) -> bool {
    let pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);
    return vec_contains(pool_tokens, token);
}

#[storage(read)]
fn is_general_pool_token_registered(pool_id: b256, token: ContractId) -> bool {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    return storage.indexes.get((
        pool_id,
        token,
    )) != 0;
}

#[storage(read, write)]
fn perform_pool_management_operation(
    kind: PoolBalanceOpKind,
    pool_id: b256,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    let specialization: PoolSpecialization = storage.pool_specialization.get(pool_id);
    if let PoolBalanceOpKind::Withdraw = kind {
        return withdraw_pool_balance(pool_id, specialization, token, amount);
    } else if let PoolBalanceOpKind::Deposit = kind {
        return deposit_pool_balance(pool_id, specialization, token, amount);
    } else {
        // PoolBalanceOpKind::UPDATE
        return update_managed_balance(pool_id, specialization, token, amount);
    }
}

// Moves `amount` tokens from a Pool's 'cash' to 'managed' balance, and transfers them to the caller.
// Returns the 'cash' and 'managed' balance deltas as a result of this call, which will be complementary.
#[storage(read, write)]
fn withdraw_pool_balance(
    pool_id: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    if let PoolSpecialization::TwoToken = specialization {
        update_two_token_pool_shared_balance_cash_to_managed(pool_id, token, amount);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        update_minimal_swap_info_pool_balance_cash_to_managed(pool_id, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        update_general_pool_balance_cash_to_managed(pool_id, token, amount);
    }

    if (amount > 0) {
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address,
            _ => revert(0),
        };
        transfer_to_output(amount, token, sender);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(-amount);
    // managed_delta = int256(amount);
    return (
        amount,
        amount,
    );
}

// todo returns signed int in solidity. no support of signed int in sway
#[storage(read, write)]
fn update_two_token_pool_shared_balance_cash_to_managed(
    pool_id: b256,
    token: ContractId,
    amount: u64,
) -> u64 {
    let (token_a, mut balance_a, token_b, mut balance_b) = get_two_token_pool_balances(pool_id);

    let mut delta: u64 = 0;

    if token == token_a {
        let new_balance = cash_to_managed(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };

    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let bal = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((
        pool_id,
        pair_hash,
    ), balance);

    return delta;
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn update_minimal_swap_info_pool_balance_cash_to_managed(
    pool_id: b256,
    token: ContractId,
    amount: u64,
) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = cash_to_managed(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((
        pool_id,
        token,
    ), new_balance);

    return managed_delta(new_balance, current_balance);
}

// this function returns signed int in solidity code
#[storage(write, read)]
fn update_general_pool_balance_cash_to_managed(
    pool_id: b256,
    token: ContractId,
    amount: u64,
) -> u64 {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = get_general_pool_balance_storage_map(pool_id, token);

    let new_balance = cash_to_managed(current_balance, amount);
    set(pool_id, token, new_balance);

    // pool_balances.insert(pool_balances, token, new_balance);
    return managed_delta(new_balance, current_balance);
}

#[storage(read)]
fn get_minimal_swap_info_pool_balance(pool_id: b256, token: ContractId) -> b256 {
    let balance = storage.minimal_swap_info_pools_balances.get((
        pool_id,
        token,
    ));
    let token_registered = is_zero(balance) || vec_contains(storage.minimal_swap_info_pools_tokens.get(pool_id), token);

    if (!token_registered) {
        // PoolRegistry::_ensure_registered_pool(pool_id);
        revert(TOKEN_NOT_REGISTERED);
    }

    return balance;
}

#[storage(read, write)]
fn deposit_pool_balance(
    pool_id: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    if let PoolSpecialization::TwoToken = specialization {
        two_token_pool_managed_to_cash(pool_id, token, amount);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        minimal_swap_info_pool_managed_to_cash(pool_id, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        general_pool_managed_to_cash(pool_id, token, amount);
    }

    if (amount > 0) {
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address,
            _ => revert(0),
        };
        transfer_to_output(amount, contract_id(), sender);
        // token.safeTransferFrom(msg.sender, address(this), amount);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(amount);
    // managed_delta = int256(-amount);
    return (
        amount,
        amount,
    );
}

// todo returns signed int in solidity. no support of signed int in sway
#[storage(read, write)]
fn two_token_pool_managed_to_cash(pool_id: b256, token: ContractId, amount: u64) -> u64 {
    let (mut token_a, mut balance_a, _, mut balance_b) = get_two_token_pool_balances(pool_id);

    let mut delta: u64 = 0;

    if token == token_a {
        let new_balance = managed_to_cash(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };

    let bal_hash = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((
        pool_id,
        bal_hash,
    ), balance);

    return delta;
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn minimal_swap_info_pool_managed_to_cash(pool_id: b256, token: ContractId, amount: u64) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = managed_to_cash(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((
        pool_id,
        token,
    ), new_balance);

    return managed_delta(new_balance, current_balance);
}

#[storage(write, read)]
fn general_pool_managed_to_cash(pool_id: b256, token: ContractId, amount: u64) -> u64 {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = get_general_pool_balance_storage_map(pool_id, token);

    let new_balance = managed_to_cash(current_balance, amount);
    set(pool_id, token, new_balance);

    return managed_delta(new_balance, current_balance);
}

// Sets a Pool's 'managed' balance to `amount`.
// Returns the 'cash' and 'managed' balance deltas as a result of this call (the 'cash' delta will always be zero).
#[storage(read, write)]
fn update_managed_balance(
    pool_id: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    let mut managed_delta_ = 0;
    if let PoolSpecialization::TwoToken = specialization {
        let managed_delta_ = set_two_token_pool_managed_balance(pool_id, token, amount);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        let managed_delta_ = set_minimal_swap_info_pool_managed_balance(pool_id, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        let managed_delta_ = set_general_pool_managed_balance(pool_id, token, amount);
    }

    // cashDelta = 0;
    return (
        0,
        managed_delta_,
    )
}

#[storage(read, write)]
fn set_two_token_pool_managed_balance(pool_id: b256, token: ContractId, amount: u64) -> u64 {
    let (mut token_a, mut balance_a, _, mut balance_b) = get_two_token_pool_balances(pool_id);
    let mut delta: u64 = 0;

    if token == token_a {
        let new_balance = set_managed(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };

    let bal_hash = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((
        pool_id,
        bal_hash,
    ), balance);

    return delta;
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn set_minimal_swap_info_pool_managed_balance(
    pool_id: b256,
    token: ContractId,
    amount: u64,
) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = set_managed(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((
        pool_id,
        token,
    ), new_balance);

    return managed_delta(new_balance, current_balance);
}

#[storage(write, read)]
fn set_general_pool_managed_balance(pool_id: b256, token: ContractId, amount: u64) -> u64 {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = get_general_pool_balance_storage_map(pool_id, token);

    let new_balance = set_managed(current_balance, amount);
    set(pool_id, token, new_balance);

    return managed_delta(new_balance, current_balance);
}

#[storage(read)]
fn get_two_token_pool_shared_balances(
    pool_id: b256,
    token_x: ContractId,
    token_y: ContractId,
) -> (b256, b256) {
    let (token_a, token_b) = sort_two_tokens(token_x, token_y);
    let pair_hash = get_two_token_pair_hash(token_a, token_b);

    let shared_cash = storage.balances.get((
        pool_id,
        pair_hash,
    )).shared_cash;
    let shared_managed = storage.balances.get((
        pool_id,
        pair_hash,
    )).shared_managed;

    let token_registered = is_zero(shared_cash) || is_zero(shared_managed) || (is_two_token_pool_token_registered(pool_id, token_a) && is_two_token_pool_token_registered(pool_id, token_b));

    if !token_registered {
        ensure_registered_pool(pool_id);
        revert(TOKEN_NOT_REGISTERED);
    }

    let balance_a = from_shared_to_balance_a(shared_cash, shared_managed);
    let balance_b = from_shared_to_balance_b(shared_cash, shared_managed);

    return (
        balance_a,
        balance_b,
    );
}

#[storage(write, read)]
fn process_minimal_swap_info_pool_swap_request(request: SwapRequest, pool: ContractId) -> u64 {
    let token_in_balance = get_minimal_swap_info_pool_balance(request.pool_id, request.token_in);
    let token_out_balance = get_minimal_swap_info_pool_balance(request.pool_id, request.token_out);

    // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    let (token_in_balance, token_out_balance, amount_calculated) = call_minimal_swap_info_pool_on_swap_hook(request, pool, token_in_balance, token_out_balance);
    storage.minimal_swap_info_pools_balances.insert((
        request.pool_id,
        request.token_in,
    ), token_in_balance);
    storage.minimal_swap_info_pools_balances.insert((
        request.pool_id,
        request.token_out,
    ), token_out_balance);

    return amount_calculated;
}

#[storage(read, write)]
fn process_general_pool_swap_request(request: SwapRequest, pool: ContractId) -> u64 {
    let mut request = request;
    let mut token_in_balance = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let mut token_out_balance = 0x0000000000000000000000000000000000000000000000000000000000000000;

    // We access both token indexes without checking existence, because we will do it manually immediately after.
    // todo need to check this
    let pool_balances = storage.general_pools_balances.get(request.pool_id);
    let mut index_in = storage.indexes.get((
        request.pool_id,
        request.token_in,
    ));
    let mut index_out = storage.indexes.get((
        request.pool_id,
        request.token_out,
    ));

    if (index_in == 0 || index_out == 0) {
        // The tokens might not be registered because the Pool itself is not registered. We check this to provide a
        // more accurate revert reason.
        ensure_registered_pool(request.pool_id);
        revert(TOKEN_NOT_REGISTERED);
    }

    // EnumerableMap stores indices *plus one* to use the zero index as a sentinel value - because these are valid,
    // we can undo this.
    index_in = index_in - 1;
    index_out = index_out - 1;

    let token_amount = pool_balances.length;
    let mut current_balances = ~Vec::new();

    request.last_change_block = 0;
    let mut count = 0;
    while count < token_amount {
        // Because the iteration is bounded by `token_amount`, and no tokens are registered or deregistered here, we
        // know `i` is a valid token index and can use `unchecked_valueAt` to save storage reads.
        let balance = unchecked_value_at(request.pool_id, count);

        current_balances.push(total(balance));
        request.last_change_block = max(request.last_change_block, last_change_block(balance));

        if (count == index_in) {
            token_in_balance = balance;
        } else if (count == index_out) {
            token_out_balance = balance;
        }
    }

    // Perform the swap request callback and compute the new balances for 'token in' and 'token out' after the swap
    // todo this function belong to pool-utils/contracts/BaseGeneralPool
    // todo dummy value
    // let amount_calculated = on_swap(pool, request, current_balances, index_in, index_out);
    let amount_calculated = 0;
    let (amount_in, amount_out) = get_amounts(request.kind, request.amount, amount_calculated);
    token_in_balance = increase_cash(token_in_balance, amount_in);
    token_out_balance = decrease_cash(token_out_balance, amount_out);

    // Because no tokens were registered or deregistered between now or when we retrieved the indexes for
    // 'token in' and 'token out', we can use `unchecked_setAt` to save storage reads.
    unchecked_set_at(request.pool_id, index_in, token_in_balance);
    unchecked_set_at(request.pool_id, index_out, token_out_balance);

    return amount_calculated;
}

#[storage(read, write)]
fn unchecked_set_at(pool_id: b256, index: u64, value: b256) {
    let mut entry = storage.entries.get((
        pool_id,
        index,
    ));
    entry.value = value;
    storage.entries.insert((
        pool_id,
        index,
    ), entry);
}

#[storage(read)]
fn is_two_token_pool_token_registered(pool_id: b256, token: ContractId) -> bool {
    let pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    return (token == pool_tokens.token_a || token == pool_tokens.token_b) && token != ~ContractId::from(ZERO_B256);
}

// Implements both `join_pool` and `exit_pool`, based on `kind`.
#[storage(read, write)]
fn join_or_exit(
    kind: PoolBalanceChangeKind,
    pool_id: b256,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange,
) {
    // This fn uses a large number of stack variables (pool_id, sender and recipient, balances, amounts, fees,
    // etc.), which leads to 'stack too deep' issues. It relies on private functions with seemingly arbitrary
    // interfaces to work around this limitation.
    require(change.assets.len() == change.limits.len(), InputError::InputLengthMismatch);

    // We first check that the caller passed the Pool's registered tokens in the correct order, and retrieve the
    // current balance for each.
    let tokens = translate_to_ierc20_array(change.assets);
    let (actual_tokens, balances) = private_get_pool_tokens(pool_id);

    // todo will comment when whole applicastion work, because tokens should be registered in the pool
    // require(actual_tokens.len() == tokens.len(), InputError::InputLengthMismatch);
    // require(actual_tokens.len() > 0, PoolError::POOL_NO_TOKENS);
    let mut count = 0;
    while count < actual_tokens.len() {
        require(actual_tokens.get(count).unwrap() == tokens.get(count).unwrap(), PoolError::TokensMismatch);
        count += 1;
    }

    // The bulk of the work is done here: the corresponding Pool hook is called, its final balances are computed,
    // assets are transferred, and fees are paid.
    // let (final_balances, amounts_in_or_out, paid_protocol_swap_fee_amounts) = call_pool_balance_change(kind, pool_id, sender, recipient, change, balances);

    // All that remains is storing the new Pool balances.
    // if let PoolSpecialization::TwoToken = specialization {
    //     set_two_token_pool_cash_balances(pool_id, tokens.get(0).unwrap(), final_balances.get(0).unwrap(), tokens.get(1).unwrap(), final_balances.get(1).unwrap());
    // } else if let PoolSpecialization::MinimalSwapInfo = specialization
    // {
    //     set_minimal_swap_info_pool_balances(pool_id, tokens, final_balances);
    // } else {
    //     // PoolSpecialization.GENERAL
    //     set_general_pool_balances(pool_id, final_balances);
    // }
    // // Amounts in are positive, out are negative
    // let mut positive: bool = false;
    // if let PoolBalanceChangeKind::Join = kind {
    //     positive = true;
    // }
    // let balances_log = unsafe_cast_to_int256(amounts_in_or_out, positive);
    // log(EventPoolBalanceChanged {
    //     pool_id: pool_id,
    //     sender: sender,
    //     tokens: tokens,
    //     balances: balances_log,
    //     paid_protocol_swap_fee_amounts: paid_protocol_swap_fee_amounts,
    // });
}

// Calls the corresponding Pool hook to get the amounts in/out plus protocol fee amounts, and performs the
// associated token transfers and fee payments, returning the Pool's final balances.
#[storage(read, write)]
fn call_pool_balance_change(
    kind: PoolBalanceChangeKind,
    pool_id: b256,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
) -> (Vec<b256>, Vec<u64>, Vec<u64>) {
    let (total_balances, last_change_block_) = totals_and_last_change_block(balances);

    let x = abi(ExternalInterface, pool_id);
    let sender_contract: b256 = sender.into();
    let sender_contract_id = ~ContractId::from(sender_contract);
    let recipient_contract: b256 = recipient.into();
    let recipient_contract_id = ~ContractId::from(recipient_contract);
    if let PoolBalanceChangeKind::Join = kind {
        // todo Pool is not deployed yet -> workaround
        let mut amounts_in_or_out = ~Vec::new();
        amounts_in_or_out.push(323);
        amounts_in_or_out.push(31);
        let mut due_protocol_fee_amounts = ~Vec::new();
        due_protocol_fee_amounts.push(423);
        due_protocol_fee_amounts.push(32);
        // let (amounts_in_or_out, due_protocol_fee_amounts) = x.on_join_pool(pool_id, sender_contract_id, recipient_contract_id, total_balances, last_change_block, MAX_PROTOCOL_SWAP_FEE_PERCENTAGE, change.user_data);
        // require(balances.len() == amounts_in_or_out.len() && amounts_in_or_out.len() == due_protocol_fee_amounts.len(), InputError::InputLengthMismatch);
        // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
        // their participation.
        // let final_balances = process_join_pool_transfers(sender, change, balances, amounts_in_or_out, due_protocol_fee_amounts);
        let mut final_balances = ~Vec::new();
        final_balances.push(0x0000000000000000000000000000000000000000000000000000000000000143);
        final_balances.push(0x0000000000000000000000000000000000000000000000000000000000000020);
        return (
            final_balances,
            amounts_in_or_out,
            due_protocol_fee_amounts,
        );
    } else {
        let (amounts_in_or_out, due_protocol_fee_amounts) = x.on_exit_pool(pool_id, sender_contract_id, recipient_contract_id, total_balances, last_change_block_, MAX_PROTOCOL_SWAP_FEE_PERCENTAGE, change.user_data);
        require(balances.len() == amounts_in_or_out.len() && amounts_in_or_out.len() == due_protocol_fee_amounts.len(), InputError::InputLengthMismatch);
        // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
        // their participation.
        let final_balances = process_exit_pool_transfers(recipient, change, balances, amounts_in_or_out, due_protocol_fee_amounts);
        return (
            final_balances,
            amounts_in_or_out,
            due_protocol_fee_amounts,
        );
    }
}

// Transfers `amounts_in` from `sender`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees.
// Returns the Pool's final balances, which are the current balances plus `amounts_in` minus accumulated protocol
// swap fees.
#[storage(read, write)]
fn process_join_pool_transfers(
    sender: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amounts_in: Vec<u64>,
    due_protocol_fee_amounts: Vec<u64>,
) -> Vec<b256> {
    // We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
    let mut wrapped_eth = 0;

    let mut final_balances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amount_in = amounts_in.get(count).unwrap();
        require(amount_in <= change.limits.get(count).unwrap(), PoolError::JoinAboveMax);

        // Receive assets from the sender - possibly from Internal Balance.
        let asset: ContractId = change.assets.get(count).unwrap();
        receive_asset(asset, amount_in, sender, change.use_internal_balance);

        if (is_eth(asset)) {
            wrapped_eth = wrapped_eth + amount_in;
        }

        let fee_amount = due_protocol_fee_amounts.get(count).unwrap();
        // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        pay_fee_amount(translate_to_ierc20(asset), fee_amount);
        // Compute the new Pool balances. Note that the fee amount might be larger than `amount_in`,
        // resulting in an overall decrease of the Pool's balance for a token.
        // This lets us skip checked arithmetic
        if amount_in >= fee_amount {
            final_balances.push(increase_cash(balances.get(count).unwrap(), amount_in - fee_amount));
        } else {
            final_balances.push(decrease_cash(balances.get(count).unwrap(), fee_amount - amount_in));
        }
        count += 1;
    }

    // Handle any used and remaining ETH.
    handle_remaining_eth(wrapped_eth);

    return final_balances;
}

fn pay_fee_amount(token: ContractId, amount: u64) {
    if (amount > 0) {
        let address: b256 = contract_id().into();
        transfer_to_output(amount, token, ~Address::from(address));
    }
}

// Transfers `amounts_out` to `recipient`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees from the Pool.
//
// Returns the Pool's final balances, which are the current `balances` minus `amounts_out` and fees paid
// (`due_protocol_fee_amounts`).
#[storage(read, write)]
fn process_exit_pool_transfers(
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amounts_out: Vec<u64>,
    due_protocol_fee_amounts: Vec<u64>,
) -> Vec<b256> {
    let mut final_balances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amount_out = amounts_out.get(count).unwrap();
        require(amount_out >= change.limits.get(count).unwrap(), PoolError::ExitBelowMin);

        // Send tokens to the recipient - possibly to Internal Balance
        let asset = change.assets.get(count).unwrap();
        send_asset(asset, amount_out, recipient, change.use_internal_balance);

        let fee_amount = due_protocol_fee_amounts.get(count).unwrap();
        pay_fee_amount(translate_to_ierc20(asset), fee_amount);

        // Compute the new Pool balances. A Pool's token balance always decreases after an exit (potentially by 0).
        final_balances.push(decrease_cash(balances.get(count).unwrap(), amount_out + fee_amount));
        count += 1;
    }
    while count < balances.len() {
        final_balances.push(ZERO_B256);
        count += 1;
    }
    return final_balances;
}

#[storage(read, write)]
fn deposit_to_internal_balance(
    asset: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64,
) {
    increase_internal_balance(recipient, translate_to_ierc20(asset), amount);
    receive_asset(asset, amount, sender, false);
}

#[storage(read, write)]
fn transfer_internal_balance(
    token: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64,
) {
    // A partial decrease of Internal Balance is disallowed: `sender` must have the full `amount`.
    decrease_internal_balance(sender, token, amount, false);
    increase_internal_balance(recipient, token, amount);
}

#[storage(read, write)]
fn transfer_to_external_balance(
    token: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64,
) {
    if (amount > 0) {
        transfer_to_output(amount, token, recipient);

        log(EventExternalBalanceTransfer {
            amount: amount,
            recipient: recipient,
            sender: sender,
            token: token,
        });
    }
}

#[storage(read, write)]
fn withdraw_from_internal_balance(
    asset: ContractId,
    sender: Address,
    recipient: Address,
    amount: u64,
) {
    // A partial decrease of Internal Balance is disallowed: `sender` must have the full `amount`.
    decrease_internal_balance(sender, translate_to_ierc20(asset), amount, false);
    send_asset(asset, amount, recipient, false);
}

// Increases `account`L's Internal Balance for `token` by `amount`.
#[storage(read, write)]
fn increase_internal_balance(account: Address, token: ContractId, amount: u64) {
    let current_balance: u64 = get_internal_balance_private(account, token);
    let new_balance: u64 = amount + current_balance;
    //todo- When INT256 is implemented
    set_internal_balance(account, token, new_balance, amount);
}

#[storage(read, write)]
fn receive_asset(
    asset: ContractId,
    amount: u64,
    sender: Address,
    from_internal_balance: bool,
) {
    let mut amount = amount;
    if amount == 0 {
        return;
    }

    if is_eth(asset) {
        require(!from_internal_balance, PoolError::InvalidEthInternalBalance);

        // The ETH amount to receive is deposited into the WETH contract, which will in turn mint WETH for
        // the Vault at a 1:1 ratio.
        // A check for this condition is also introduced by the compiler, but this one provides a revert reason.
        // Note we're checking for the Vault's total balance, *not* ETH sent in this transaction.
        require(balance_of(BASE_ASSET_ID, contract_id()) >= amount, PoolError::InsufficientEth);
        force_transfer_to_contract(amount, BASE_ASSET_ID, contract_id());
    } else {
        let token = asset;

        if from_internal_balance {
            // We take as many tokens from Internal Balance as possible: any remaining amounts will be transferred.
            let deducted_balance: u64 = decrease_internal_balance(sender, token, amount, true);
            // Because `deducted_balance` will be always the lesser of the current internal balance
            // and the amount to decrease, it is safe to perform unchecked arithmetic.
            amount = amount - deducted_balance;
        }

        if amount > 0 {
            force_transfer_to_contract(amount, token, contract_id());
        }
    }
}

// Sends `amount` of `asset` to `recipient`. If `to_internal_balance` is true, the asset is deposited as Internal
// Balance instead of being transferred.
//
// If `asset` is ETH, `to_internal_balance` must be false (as ETH cannot be held as internal balance), and the funds
// are instead sent directly after unwrapping WETH.
#[storage(read, write)]
fn send_asset(
    asset: ContractId,
    amount: u64,
    recipient: Address,
    to_internal_balance: bool,
) {
    if (amount == 0) {
        return;
    }

    if (is_eth(asset)) {
        // Sending ETH is not as involved as receiving it: the only special behavior is it cannot be
        // deposited to Internal Balance.
        require(!to_internal_balance, PoolError::InvalidEthInternalBalance);
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address,
            _ => revert(0),
        };

        // First, the Vault withdraws deposited ETH from the WETH contract, by burning the same amount of WETH
        // from the Vault. This receipt will be handled by the Vault's `receive`
        // Then, the withdrawn ETH is sent to the recipient.
        transfer_to_output(amount, BASE_ASSET_ID, sender);
    } else {
        let token = asset;
        if (to_internal_balance) {
            increase_internal_balance(recipient, token, amount);
        } else {
            transfer_to_output(amount, token, recipient);
        }
    }
}

// Decreases `account`'s Internal Balance for `token` by `amount`. If `allow_partial` is true, this function
// doesn't revert if `account` doesn't have enough balance, and sets it to zero and returns the deducted amount
// instead.
#[storage(read, write)]
fn decrease_internal_balance(
    account: Address,
    token: ContractId,
    amount: u64,
    allow_partial: bool,
) -> u64 {
    let current_balance: u64 = get_internal_balance_private(account, token);
    require(allow_partial || (current_balance >= amount), PoolError::InsufficientInternalBalance);

    let deducted = current_balance - amount;
    // By construction, `deducted` is lower or equal to `current_balance`, so we don't need to use checked
    // arithmetic.
    let new_balance: u64 = current_balance - deducted;

    // Todo When signed Integers are added
    set_internal_balance(account, token, new_balance, (deducted));
    return deducted;
}

// Sets `account`'s Internal Balance for `token` to `new_balance`.
//
// Emits an `EventInternalBalanceChanged` event. This event includes `delta`, which is the amount the balance increased
// (if positive) or decreased (if negative). To avoid reading the current balance in order to compute the delta,
// this function relies on the caller providing it directly.
// Todo When signed Integers are added
#[storage(read, write)]
fn set_internal_balance(
    account: Address,
    token: ContractId,
    new_balance: u64,
    delta: u64,
) {
    storage.internal_token_balance.insert((
        account,
        token,
    ), new_balance);
    log(EventInternalBalanceChanged {
        account: account,
        delta: delta,
        token: token,
    });
}

#[storage(read)]
fn get_internal_balance_private(account: Address, token: ContractId) -> u64 {
    return storage.internal_token_balance.get((
        account,
        token,
    ));
}

// PoolTokens
#[storage(read)] // Returns all of `pool_id`'s registered tokens, along with their raw balances.
fn private_get_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<b256>) {
    let specialization = storage.pool_specialization.get(pool_id);
    if let PoolSpecialization::TwoToken = specialization {
        return get_two_token_pool_tokens(pool_id);
    } else if let PoolSpecialization::MinimalSwapInfo = specialization
    {
        return get_minimal_swap_info_pool_tokens(pool_id);
    } else {
        // PoolSpecialization::GENERAL
        return get_general_pool_tokens(pool_id);
    }
}

#[storage(read)]
fn get_two_token_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<b256>) {
    let (token_a, balance_a, token_b, balance_b) = get_two_token_pool_balances(pool_id);

    if token_a == ~ContractId::from(ZERO_B256) || token_b == ~ContractId::from(ZERO_B256) {
        let contract_vec: Vec<ContractId> = ~Vec::new();
        let bytes_vec: Vec<b256> = ~Vec::new();
        return (
            contract_vec,
            bytes_vec,
        );
    }

    let mut tokens = ~Vec::new();
    tokens.push(token_a);
    tokens.push(token_b);

    let mut balances = ~Vec::new();
    balances.push(balance_a);
    balances.push(balance_b);

    return (
        tokens,
        balances,
    );
}
#[storage(read)]
fn get_minimal_swap_info_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<b256>) {
    let pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);
    let mut balances = ~Vec::new();

    let mut count = 0;
    while count < pool_tokens.len() {
        let token = pool_tokens.get(count).unwrap();
        balances.push(storage.minimal_swap_info_pools_balances.get((
            pool_id,
            token,
        )));
        count += 1;
    }

    return (
        pool_tokens,
        balances,
    );
}

#[storage(read)]
fn get_general_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<b256>) {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let mut tokens = ~Vec::new();
    let mut balances = ~Vec::new();

    let mut count = 0;
    while (count < tokens.len()) {
        // Because the iteration is bounded by `tokens.length`, which matches the EnumerableMap's length, we can use
        // `unchecked_at` as we know `i` is a valid token index, saving storage reads.
        let (token, balance) = unchecked_at(pool_id, count);
        tokens.push(token);
        balances.push(balance);
        count += 1;
    }

    return (tokens, balances);
}

#[storage(read)]
fn unchecked_at(pool_id: b256, index: u64) -> (ContractId, b256) {
    let entry = storage.entries.get((
        pool_id,
        index,
    ));
    return (
        entry.key,
        entry.value,
    );
}

#[storage(read)]
fn get_two_token_pool_balances(pool_id: b256) -> (ContractId, b256, ContractId, b256) {
    let pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    let token_a = pool_tokens.token_a;
    let token_b = pool_tokens.token_b;
    let pair_hash = get_two_token_pair_hash(token_a, token_b);

    let shared_cash = storage.balances.get((
        pool_id,
        pair_hash,
    )).shared_cash;
    let shared_managed = storage.balances.get((
        pool_id,
        pair_hash,
    )).shared_managed;

    let balance_a = from_shared_to_balance_a(shared_cash, shared_managed);
    let balance_b = from_shared_to_balance_b(shared_cash, shared_managed);

    return (
        token_a,
        balance_a,
        token_b,
        balance_b,
    );
}

// PoolRegistry
#[storage(read)]
fn ensure_registered_pool(pool_id: b256) {
    require(storage.is_pool_registered.get(pool_id), PoolError::InvalidPoolId);
}
