contract;

dep errors;
dep data_structures;
dep interface;
dep utils;
dep events;
dep ops;

use errors::Error;
use data_structures::{
    abi_encode,
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
    UserData,
};

use interface::{ExternalInterface, vault};
use utils::{
    cash,
    cash_to_managed,
    decrease_cash,
    from_shared_to_balance_a,
    from_shared_to_balance_b,
    get_amounts,
    get_pool_address,
    get_pool_specialization,
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
    to_pool_id,
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
    AuthorizerChanged,
    ExternalBalanceTransfer,
    FlashLoan,
    InternalBalanceChanged,
    PoolBalanceChanged,
    PoolBalanceManaged,
    PoolRegistered,
    RelayerApprovalChanged,
    TokensDeregistered,
    TokensRegistered,
};

use ops::get_word_from_b256;

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
        call_frames::{
            contract_id,
            msg_asset_id,
        },
        msg_amount,
    },
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    math::*,
    option::Option,
    reentrancy::is_reentrant,
    result::*,
    revert::{
        require,
        revert,
    },
    storage::{
        get,
        StorageMap,
        store,
    },
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
const ZERO_ADDRESS: ContractId = ContractId {
    value: 0x0000000000000000000000000000000000000000000000000000000000000000,
};
const TOKEN_ALREADY_REGISTERED = 522;
const PAUSE_WINDOW_END_TIME = 1;
const BUFFER_PERIOD_END_TIME = 2;

pub struct TwoTokenPoolTokens1 {
    token_a: b256,
    token_b: b256,
    // workaround of nested storageMap
    // balances: StorageMap<b256, TwoTokenPoolBalances>,
    balances: b256,
}
struct GetPoolTokens{
    tk1: b256,
    tk2: b256,
    am1: u64,
    am2: u64,
    ts: u64,
}
storage {
    two_token_pool_tokens: StorageMap<b256, TwoTokenPoolTokens> = StorageMap {},
    two_token_pool_tokens1: StorageMap<b256, TwoTokenPoolTokens1> = StorageMap {},
    // first b256 value is the pool id sencond b256 value is balances
    balances: StorageMap<(b256, b256), TwoTokenPoolBalances> = StorageMap {},
    internal_token_balance: StorageMap<(Address, ContractId), u64> = StorageMap {},
    minimal_swap_info_pools_balances: StorageMap<(b256, ContractId), b256> = StorageMap {},
    minimal_swap_info_pools_tokens: StorageMap<b256, Vec<ContractId>> = StorageMap {},
    general_pools_balances: StorageMap<b256, IERC20ToBytes32Map> = StorageMap {},
    entries: StorageMap<(b256, u64), IERC20ToBytes32MapEntry> = StorageMap {},
    indexes: StorageMap<(b256, ContractId), u64> = StorageMap {},
    // pool_asset_managers: StorageMap<(b256, b256), b256> = StorageMap {},
    pool_asset_managers: StorageMap<(b256, ContractId), Address> = StorageMap {},
    swap_fee_percentage: u64 = 1000000,
    flash_loan_fee_percentage: u64 = 1000000,
    is_pool_registered: StorageMap<b256, bool> = StorageMap {},
    next_pool_nonce: u64 = 0,
    approved_relayers: StorageMap<(Address, Address), bool> = StorageMap {},
    authorizer: ContractId = ContractId {
        value: 0xa3f865aa351e51cfeb40f5178d1564bb629fe9030b83caf6361d1baaf5b90b5a,
    },
    weth_contract_id: ContractId = ContractId {
        value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    },
    authorizer_contract_id: ContractId = ContractId {
        value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    },
    next_nonce: StorageMap<Address, u64> = StorageMap {},
    paused: bool = false,
    pool_specialization: StorageMap<b256, PoolSpecialization> = StorageMap {},
}

impl vault for Contract {
    // #[storage(read, write)]
    // fn batch_swap(
    //     kind: SwapKind,
    //     swaps: Vec<BatchSwapStep>,
    //     assets: Vec<ContractId>,
    //     funds: FundManagement,
    //     limits: Vec<u64>,
    //     deadline: u64,
    // ) -> Vec<u64> {
    //     // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
    //     // solhint-disable-next-line not-rely-on-time
    //     // require(block_timestamp() <= deadline, Error::SWAP_DEADLINE);
    //     require(assets.len() == limits.len(), Error::INPUT_LENGTH_MISMATCH);
    //     // Perform the swaps, updating the Pool token balances and computing the net Vault asset deltas.
    //     let assetDeltas = swap_with_pool(swaps, assets, funds, kind);
    //     // Process asset deltas, by either transferring assets from the sender (for positive deltas) or to the recipient
    //     // (for negative deltas).
    //     let mut wrappedEth = 0;
    //     let mut count = 0;
    //     while count < assets.len() {
    //         let asset = assets.get(count).unwrap();
    //         let delta = assetDeltas.get(count).unwrap();
    //         require(delta <= limits.get(count).unwrap(), Error::SWAP_LIMIT);
    //         if (delta > 0) {
    //             let toReceive = delta;
    //             receive_asset(asset, toReceive, funds.sender, funds.fromInternalBalance);
    //             if (is_eth(asset)) {
    //                 wrappedEth = wrappedEth + toReceive;
    //             }
    //         } else if (delta < 0) {
    //             // let toSend = -delta;
    //             let toSend = delta;
    //             send_asset(asset, toSend, funds.recipient, funds.toInternalBalance);
    //         }
    //     }
    //     // Handle any used and remaining ETH.
    //     handle_remaining_eth(wrappedEth);
    //     return assetDeltas;
    // }
    // #[storage(read, write)]
    // fn swap(
    //     singleSwap: SingleSwap,
    //     funds: FundManagement,
    //     limit: u64,
    //     deadline: u64,
    // ) -> u64 {
    //     // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
    //     // solhint-disable-next-line not-rely-on-time
    //     // require(block_timestamp() <= deadline, Error::SWAP_DEADLINE);
    //     // This revert reason is for consistency with `batch_swap`: an equivalent `swap` performed using that fn
    //     // would result in this error.
    //     require(singleSwap.amount > 0, Error::UNKNOWN_AMOUNT_IN_FIRST_SWAP);
    //     let tokenIn = translate_to_ierc20(singleSwap.assetIn);
    //     let tokenOut = translate_to_ierc20(singleSwap.assetOut);
    //     require(tokenIn != tokenOut, Error::CANNOT_SWAP_SAME_TOKEN);
    //     // Initializing each struct field one-by-one uses less gas than setting all at once.
    //     let poolRequest = SwapRequest {
    //         poolId: singleSwap.poolId,
    //         kind: singleSwap.kind,
    //         tokenIn: tokenIn,
    //         tokenOut: tokenOut,
    //         amount: singleSwap.amount,
    //         userData: singleSwap.userData,
    //         from: funds.sender,
    //         to: funds.recipient,
    //         // todo The lastChangeBlock field is left uninitialized.
    //         lastChangeBlock: 0,
    //     };
    //     let amountIn = 0;
    //     let amountOut = 0;
    //     let (amountCalculated, amountIn, amountOut) = swap_with_pool_hook(poolRequest);
    //     if let SwapKind::GIVEN_IN = singleSwap.kind {
    //         require(amountOut >= limit, Error::SWAP_LIMIT);
    //     } else {
    //         require(amountIn <= limit, Error::SWAP_LIMIT);
    //     }
    //     receive_asset(singleSwap.assetIn, amountIn, funds.sender, funds.fromInternalBalance);
    //     send_asset(singleSwap.assetOut, amountOut, funds.recipient, funds.toInternalBalance);
    //     // If the asset in is ETH, then `amountIn` ETH was wrapped into WETH.
    //     if is_eth(singleSwap.assetIn) {
    //         handle_remaining_eth(amountIn);
    //     } else {
    //         handle_remaining_eth(0);
    //     }
    //     return amountCalculated;
    // }
    // // This fn is not marked as `nonReentrant` because the underlying mechanism relies on reentrancy
    // #[storage(write, read)]
    // fn query_batch_swap(
    //     kind: SwapKind,
    //     swaps: Vec<BatchSwapStep>,
    //     assets: Vec<ContractId>,
    //     funds: FundManagement,
    // ) -> Vec<u64> {
    //     // In order to accurately 'simulate' swaps, this fn actually does perform the swaps, including calling the
    //     // Pool hooks and updating balances in storage. However, once it computes the final Vault Deltas, it
    //     // reverts unconditionally, returning this array as the revert data.
    //     //
    //     // By wrapping this reverting call, we can decode the deltas 'returned' and return them as a normal Solidity
    //     // fn would. The only caveat is the fn becomes non-view, but off-chain clients can still call it
    //     // via eth_call to get the expected result.
    //     //
    //     // This technique was inspired by the work from the Gnosis team in the Gnosis Safe contract:
    //     // https://github.com/gnosis/safe-contracts/blob/v1.2.0/contracts/GnosisSafe.sol#L265
    //     //
    //     // Most of this fn is implemented using inline assembly, as the actual work it needs to do is not
    //     // significant, and Solidity is not particularly well-suited to generate this behavior, resulting in a large
    //     // amount of generated bytecode.
    //     // let sender = match msg_sender().unwrap() {
    //     //     Identity::Address(address) => address, _ => revert(0),
    //     // };
    //     // let sender: b256 = sender.into();
    //     // let this_contract: b256 = contract_id().into();
    //     // if sender != this_contract {
    //     // We perform an external call to ourselves, forwarding the same calldata. In this call, the else clause of
    //     // the preceding if statement will be executed instead.
    //     // solhint-disable-next-line avoid-low-level-calls
    //     // let(success, _) = contract_id().call(msg.data);
    //     // // solhint-disable-next-line no-inline-assembly
    //     // assembly {
    //     //     // This call should always revert to decode the actual asset deltas from the revert reason
    //     //     switch success
    //     //         case 0 {
    //     //             // Note we are manually writing the memory slot 0. We can safely overwrite whatever is
    //     //             // stored there as we take full control of the execution and then immediately return.
    //     //             // We copy the first 4 bytes to check if it matches with the expected signature, otherwise
    //     //             // there was another revert reason and we should forward it.
    //     //             returndatacopy(0, 0, 0x04)
    //     //             let error := and(mload(0), 0xffffffff00000000000000000000000000000000000000000000000000000000)
    //     //             // If the first 4 bytes don't match with the expected signature, we forward the revert reason.
    //     //             if eq(eq(error, 0xfa61cc1200000000000000000000000000000000000000000000000000000000), 0) {
    //     //                 returndatacopy(0, 0, returndatasize())
    //     //                 revert(0, returndatasize())
    //     //             }
    //     //             // The returndata contains the signature, followed by the raw memory representation of an array:
    //     //             // length + data. We need to return an ABI-encoded representation of this array.
    //     //             // An ABI-encoded array contains an additional field when compared to its raw memory
    //     //             // representation: an offset to the location of the length. The offset itself is 32 bytes long,
    //     //             // so the smallest value we  can use is 32 for the data to be located immediately after it.
    //     //             mstore(0, 32)
    //     //             // We now copy the raw memory array from returndata into memory. Since the offset takes up 32
    //     //             // bytes, we start copying at address 0x20. We also get rid of the error signature, which takes
    //     //             // the first four bytes of returndata.
    //     //             let size := sub(returndatasize(), 0x04)
    //     //             returndatacopy(0x20, 0x04, size)
    //     //             // We finally return the ABI-encoded array, which has a total length equal to that of the array
    //     //             // (returndata), plus the 32 bytes for the offset.
    //     //             return(0, add(size, 32))
    //     //         }
    //     //         default {
    //     //             // This call should always revert, but we fail nonetheless if that didn't happen
    //     //             invalid()
    //     //         }
    //     // }
    //     // } else {
    //     //     let deltas = swap_with_pool(swaps, assets, funds, kind);
    //     // // solhint-disable-next-line no-inline-assembly
    //     // assembly {
    //     //     // We will return a raw representation of the array in memory, which is composed of a 32 byte length,
    //     //     // followed by the 32 byte int256 values. Because revert expects a size in bytes, we multiply the array
    //     //     // length (stored at `deltas`) by 32.
    //     //     let size := mul(mload(deltas), 32)
    //     //     // We send one extra value for the error signature "QueryError(int256[])" which is 0xfa61cc12.
    //     //     // We store it in the previous slot to the `deltas` array. We know there will be at least one available
    //     //     // slot due to how the memory scratch space works.
    //     //     // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
    //     //     mstore(sub(deltas, 0x20), 0x00000000000000000000000000000000000000000000000000000000fa61cc12)
    //     //     let start := sub(deltas, 0x04)
    //     //     // When copying from `deltas` into returndata, we copy an additional 36 bytes to also return the array's
    //     //     // length and the error signature.
    //     //     revert(start, add(size, 36))
    //     // }
    //     // }
    //     let deltas = swap_with_pool(swaps, assets, funds, kind);
    //     return deltas;
    // }
    #[storage(read, write)]
    fn register_tokens(
        poolId: b256,
        // tokens: Vec<ContractId>,
        tokens1: [b256; 8],
        // assetManagers: Vec<Address>,
        assetManagers1: [b256; 8],
    ) {
        is_reentrant();

        // let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
        // x.ensure_not_paused();
        // require(tokens.len() == assetManagers.len(), Error::INPUT_LENGTH_MISMATCH);
        let mut count = 0;
        let mut tokens = ~Vec::new();
        let mut assetManagers = ~Vec::new();
        while count < 8 {
            if tokens1[count] == 0x0000000000000000000000000000000000000000000000000000000000000000
            {
                break;
            }
            tokens.push(~ContractId::from(tokens1[count]));
            assetManagers.push(~Address::from(assetManagers1[count]));
            count = count + 1;
        }

        // Validates token addresses and assigns Asset Managers
        let mut count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            require(token != ~ContractId::from(ZERO_B256), Error::INVALID_TOKEN);
            storage.pool_asset_managers.insert((poolId, token, ), assetManagers.get(count).unwrap());
            count = count + 1;
        }
        
        // let token = tokens[0];
        // require(token != (ZERO_B256), Error::INVALID_TOKEN);
        // storage.pool_asset_managers.insert((
        //     poolId,
        //     token,
        // ), assetManagers[0]);
        // let token = tokens[1];
        // require(token != (ZERO_B256), Error::INVALID_TOKEN);
        // storage.pool_asset_managers.insert((
        //     poolId,
        //     token,
        // ), assetManagers[1]);
        // let specialization: PoolSpecialization = get_pool_specialization(poolId);
        let specialization = storage.pool_specialization.get(poolId);
        if let PoolSpecialization::TWO_TOKEN = specialization {
            // require(tokens.len() == 2, PoolError::TokensLengthMustBe2);
            register_two_token_pool_tokens(poolId, tokens.get(0).unwrap(), tokens.get(1).unwrap());
        } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
        {
            register_minimal_swap_info_pool_tokens(poolId, tokens);
        } else {
            register_general_pool_tokens(poolId, tokens);
        }

        // log(TokensRegistered {
        //     pool_id: poolId,
        //     tokens: tokens,
        //     asset_managers: assetManagers,
        // });
    }



    // #[storage(read, write)]
    // fn deregister_tokens(poolId: b256, tokens: Vec<ContractId>) {
    //     is_reentrant();
    //     // let x = abi(TemporarilyPausable, storage.temporarily_pausable_contract_id);
    //     // x.ensure_not_paused();
    //     let specialization: PoolSpecialization = get_pool_specialization(poolId);
    //     if let PoolSpecialization::TWO_TOKEN = specialization {
    //         require(tokens.len() == 2, Error::TOKENS_LENGTH_MUST_BE_2);
    //         deregister_two_token_pool_tokens(poolId, tokens.get(0).unwrap(), tokens.get(1).unwrap());
    //     } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    //     {
    //         deregister_minimal_swap_info_pool_tokens(poolId, tokens);
    //     } else {
    //         // PoolSpecialization::GENERAL
    //         deregister_general_pool_tokens(poolId, tokens);
    //     }
    //     // The deregister calls above ensure the total token balance is zero. Therefore it is now safe to remove any
    //     // associated Asset Managers, since they hold no Pool balance.
    //     // Todo need to be implemented when we can remove things from storage
    //     let mut count = 0;
    //     while count < tokens.len() {
    //         storage.pool_asset_managers.insert((
    //             poolId,
    //             tokens.get(count).unwrap(),
    //         ), ~Address::from(ZERO_B256));
    //         count = count + 1;
    //     }
    //     log(TokensDeregistered {
    //         poolId: poolId,
    //         tokens: tokens,
    //     });
    // }
    // PoolBalances
    #[storage(read, write)]
    fn join_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest,
    ) {
        let specialization = storage.pool_specialization.get(poolId);
        // This fn doesn't have the nonReentrant modifier: it is applied to `join_or_exit` instead.
        // Note that `recipient` is not actually payable in the context of a join - we cast it because we handle both
        // joins and exits at once.
        join_or_exit(PoolBalanceChangeKind::JOIN, poolId, specialization, sender, recipient, join_to_pool_balance_change(request));
    }

    #[storage(read, write)]
    fn exit_pool(
        poolId: b256,
        specialization: PoolSpecialization,
        sender: Address,
        recipient: Address,
        request: ExitPoolRequest,
    ) {
        // This fn doesn't have the nonReentrant modifier: it is applied to `join_or_exit` instead.
        join_or_exit(PoolBalanceChangeKind::EXIT, poolId, specialization, sender, recipient, exit_to_pool_balance_change(request));
    }

    // // Flashloans
    // #[storage(read, write)]
    // fn flash_loan(
    //     recipient: ContractId,
    //     tokens: Vec<ContractId>,
    //     amounts: Vec<u64>,
    //     userData: Vec<b256>,
    // ) {
    //     require(tokens.len() == amounts.len(), Error::INPUT_LENGTH_MISMATCH);
    //     let mut feeAmounts = ~Vec::new();
    //     let mut preLoanBalances = ~Vec::new();
    //     // Used to ensure `tokens` is sorted in ascending order, which ensures token uniqueness.
    //     let mut previousToken: ContractId = ~ContractId::from(ZERO_B256);
    //     let firstToken = previousToken;
    //     let mut count = 0;
    //     while count < tokens.len() {
    //         let token = tokens.get(count).unwrap();
    //         let amount = amounts.get(count).unwrap();
    //         if token == firstToken {
    //             let token: b256 = token.into();
    //             let previousToken: b256 = previousToken.into();
    //             require(token > previousToken, Error::ZERO_TOKEN);
    //         } else {
    //             let token: b256 = token.into();
    //             let previousToken: b256 = previousToken.into();
    //             require(token > previousToken, Error::UNSORTED_TOKENS);
    //         }
    //         previousToken = token;
    //         preLoanBalances.push(balance_of(token, contract_id()));
    //         // let x = abi(ProtocolFeesCollector, storage.protocol_fees_collector_contract_id);
    //         feeAmounts.push(calculate_flash_loan_fee_amount(amount));
    //         require(preLoanBalances.get(count).unwrap() >= amount, Error::INSUFFICIENT_FLASH_LOAN_BALANCE);
    //         // todo need to discuss this
    //         force_transfer_to_contract(amount, token, recipient);
    //         count = count + 1;
    //     }
    //     // recipient.receiveFlashLoan(tokens, amounts, feeAmounts, userData);
    //     // ! implimnetation of above function
    //     // require(msg.sender == contract_id(), Error::CALLER_NOT_VAULT);
    //     force_transfer_to_contract(amounts.get(0).unwrap(), tokens.get(0).unwrap(), contract_id());
    //     count = 0;
    //     while count < tokens.len() {
    //         let token = tokens.get(count).unwrap();
    //         let preLoanBalance = preLoanBalances.get(count).unwrap();
    //         // Checking for loan repayment first (without accounting for fees) makes for simpler debugging, and results
    //         // in more accurate revert reasons if the flash loan protocol fee percentage is zero.
    //         let postLoanBalance = balance_of(contract_id(), token);
    //         require(postLoanBalance >= preLoanBalance, Error::INVALID_POST_LOAN_BALANCE);
    //         // No need for checked arithmetic since we know the loan was fully repaid.
    //         let receivedFeeAmount = postLoanBalance - preLoanBalance;
    //         require(receivedFeeAmount >= feeAmounts.get(count).unwrap(), Error::INSUFFICIENT_FLASH_LOAN_FEE_AMOUNT);
    //         pay_fee_amount(token, receivedFeeAmount);
    //         log(FlashLoan {
    //             recipient: recipient,
    //             token: token,
    //             amount: amounts.get(count).unwrap(),
    //             receivedFeeAmount: receivedFeeAmount,
    //         });
    //         count = count + 1;
    //     }
    // }
    // // AssetManagers
    // #[storage(read, write)]
    // fn manage_pool_balance(ops: Vec<PoolBalanceOp>) {
    //     // This variable could be declared inside the loop, but that causes the compiler to allocate memory on each
    //     // loop iteration, increasing gas costs.
    //     let mut count = 0;
    //     while count < ops.len() {
    //         // By indexing the array only once, we don't spend extra gas in the same bounds check.
    //         let op: PoolBalanceOp = ops.get(count).unwrap();
    //         let pool_id: b256 = op.poolId;
    //         // let x = abi(PoolRegistry, pool_registry_contract_id);
    //         // x.ensure_registered_pool(poolId);
    //         let sender: Result<Identity, AuthError> = msg_sender();
    //         let sender: Address = match sender.unwrap() {
    //             Identity::Address(addr) => {
    //                 addr
    //             },
    //             _ => {
    //                 revert(0);
    //             },
    //         };
    //         let token: ContractId = op.token;
    //         require(is_token_registered(pool_id, token), Error::TOKEN_NOT_REGISTERED);
    //         require(storage.pool_asset_managers.get((
    //             pool_id,
    //             token,
    //         )) == sender, Error::SENDER_NOT_ASSET_MANAGER);
    //         let kind: PoolBalanceOpKind = op.kind;
    //         let amount = op.amount;
    //         let (cash_delta, managed_delta_value) = perform_pool_management_operation(kind, pool_id, token, amount);
    //         count = count + 1;
    //         log(PoolBalanceManaged {
    //             pool_id: pool_id,
    //             sender: sender,
    //             token: token,
    //             cash_delta: cash_delta,
    //             managed_delta: managed_delta_value,
    //         });
    //     }
    // }
    // #[storage(read, write)]
    // fn manage_user_balance(ops: Vec<UserBalanceOp>) {
    //     // is_reentrant();
    //     // We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
    //     let mut ethWrapped: u64 = 0;
    //     // Cache for these checks so we only perform them once (if at all).
    //     let checkedCallerIsRelayer = false;
    //     let mut checkedNotPaused = false;
    //     let mut i = 0;
    //     while i < ops.len() {
    //         let ops_value: (UserBalanceOpKind, ContractId, u64, Address, Address) = validate_user_balance_op(ops.get(i).unwrap(), checkedCallerIsRelayer);
    //         let (kind, asset, amount, sender, recipient, checkedCallerIsRelayer) = ops_value;
    //         if let UserBalanceOpKind::WITHDRAW_INTERNAL = kind {
    //             // Internal Balance withdrawals can always be performed by an authorized account.
    //             withdraw_from_internal_balance(asset, sender, recipient, amount);
    //         } else {
    //             // All other operations are blocked if the contract is paused.
    //             // We cache the result of the pause check and skip it for other operations in this same transaction
    //             // (if any).
    //             if (!checkedNotPaused) {
    //                 // let x = abi(TemporarilyPausable, temporarily_pausable_contract_id);
    //                 // x._ensure_not_paused();
    //                 checkedNotPaused = true;
    //             }
    //             if let UserBalanceOpKind::DEPOSIT_INTERNAL = kind {
    //                 deposit_to_internal_balance(asset, sender, recipient, amount);
    //                 // Keep track of all ETH wrapped into WETH as part of a deposit.
    //                 if (is_eth(asset)) {
    //                     ethWrapped = ethWrapped + amount;
    //                 }
    //             } else {
    //                 // Transfers don't support ETH.
    //                 require(!is_eth(asset), Error::CANNOT_USE_ETH_SENTINEL);
    //                 let token = asset;
    //                 if let UserBalanceOpKind::TRANSFER_INTERNAL = kind {
    //                     transfer_internal_balance(token, sender, recipient, amount);
    //                 } else {
    //                     // TRANSFER_EXTERNAL
    //                     transfer_to_external_balance(token, sender, recipient, amount);
    //                 }
    //             }
    //         }
    //     }
    //     handle_remaining_eth(ethWrapped);
    // }
    #[storage(read, write)]
    fn register_pool(poolId: b256, specialization: PoolSpecialization) -> b256 {
        // Each Pool is assigned a unique ID based on an incrementing nonce. This assumes there will never be more than
        // 2//80 Pools, and the nonce will not overflow.
        // todo msg_sender reverting the contract.
        // let address = match msg_sender().unwrap() {
        //     Identity::Address(address) => address, _ => revert(0),
        // };
        let address = ~Address::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);

        // let poolId: b256 = to_pool_id(address, specialization, (storage.next_pool_nonce));
        require(!(storage.is_pool_registered.get(poolId)), Error::INVALID_POOL_ID); // Should never happen as Pool IDs are unique.
        storage.pool_specialization.insert(poolId, specialization);
        storage.is_pool_registered.insert(poolId, true);

        storage.next_pool_nonce = storage.next_pool_nonce + 1;

        // log(PoolRegistered {
        //     poolId: poolId, address: address, specialization: specialization, 
        // }
        // );
        return poolId;
    }

    // #[storage(read, write)]
    // fn set_authorizer(newAuthorizer: ContractId) {
    //     is_reentrant();
    //     storage.authorizer = newAuthorizer;
    //     log(AuthorizerChanged {
    //         newAuthorizer: newAuthorizer,
    //     });
    // }
    // #[storage(read, write)]
    // fn set_relayer_approval(sender: Address, relayer: Address, approved: bool) {
    //     is_reentrant();
    //     storage.approved_relayers.insert((
    //         sender,
    //         relayer,
    //     ), approved);
    //     log(RelayerApprovalChanged {
    //         relayer: relayer,
    //         sender: sender,
    //         approved: approved,
    //     });
    // }
    // // TODO: View functions
    // #[storage(read)]
    // fn weth() -> ContractId {
    //     return storage.weth_contract_id;
    // }
    // #[storage(read)]
    // fn get_authorizer() -> ContractId {
    //     return storage.authorizer_contract_id;
    // }
    // #[storage(read)]
    // fn get_internal_balance(user: Address, tokens: Vec<ContractId>) -> Vec<u64> {
    //     let mut balances: Vec<u64> = ~Vec::new();
    //     let mut i = 0;
    //     while i < tokens.len() {
    //         balances.push(get_internal_balance_private(user, tokens.get(i).unwrap()));
    //         i = i + 1;
    //     }
    //     return balances;
    // }
    // #[storage(read)]
    // fn get_next_nonce(user: Address) -> u64 {
    //     return storage.next_nonce.get(user);
    // }
    // // Returns the current contract pause status, as well as the end times of the Pause Window and Buffer Period.
    // #[storage(read)]
    // fn get_paused_state() -> (bool, u64, u64, ) {
    //     return (
    //         !is_not_paused(),
    //         PAUSE_WINDOW_END_TIME,
    //         BUFFER_PERIOD_END_TIME,
    //     );
    // }
    // #[storage(read)]
    // fn get_pool(poolId: b256) -> (ContractId, PoolSpecialization) {
    //     return (
    //         ~ContractId::from(poolId),
    //         get_pool_specialization(poolId),
    //     );
    // }

    #[storage(read)]
    fn get_pool_tokens(poolId: b256) -> GetPoolTokens {
        let(tokens, rawBalances) = private_get_pool_tokens(poolId);
        let(balances, lastChangeBlock) = totals_and_last_change_block(rawBalances);
        let tk1: b256 = tokens.get(0).unwrap().into();
        let tk2: b256 = tokens.get(1).unwrap().into();

        let res = GetPoolTokens {
            tk1: tk1,
            tk2: tk2,
            am1: balances.get(0).unwrap(),
            am2: balances.get(1).unwrap(),
            ts: lastChangeBlock,
        };
        return res; 
    }
    // #[storage(read)]
    // fn get_pool_token_info(poolId: b256, token: ContractId) -> (u64, u64, u64, Address) {
    //     // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
    //     // x.with_registered_pool(poolId);
    //     let mut balance: b256 = ZERO_B256;
    //     let specialization: PoolSpecialization = get_pool_specialization(poolId);
    //     if let PoolSpecialization::TWO_TOKEN = specialization {
    //         balance = get_two_token_pool_balance(poolId, token);
    //     } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    //     {
    //         balance = get_minimal_swap_info_pool_balance(poolId, token);
    //     } else {
    //         // PoolSpecialization::GENERAL
    //         balance = get_general_pool_balance(poolId, token);
    //     }
    //     return (
    //         cash(balance),
    //         managed(balance),
    //         last_change_block(balance),
    //         storage.pool_asset_managers.get((
    //             poolId,
    //             token,
    //         )),
    //     )
    // }
    // #[storage(read)]
    // fn has_approved_relayer(user: Address, relayer: Address) -> bool {
    //     return storage.approved_relayers.get((
    //         user,
    //         relayer,
    //     ));
    // }
    // #[storage(read)]fn get_swap_fee_percentage() -> u64 {
    //     return storage.swap_fee_percentage;
    // }
}

// Returns the balance of a token in a General Pool.
// This function assumes `poolId` exists and corresponds to the General specialization setting.
// Requirements:
// - `token` must be registered in the Pool
#[storage(read)]
fn get_general_pool_balance(poolId: b256, token: ContractId) -> b256 {
    let poolBalances = storage.general_pools_balances.get(poolId);
    return get_value(poolId, token);
}

#[storage(read)]
fn get_two_token_pool_balance(pool_id: b256, token: ContractId) -> b256 {
    let (token_a, balance_a, token_b, balance_b) = get_two_token_pool_balances(pool_id);
    if token == token_a {
        return balance_a;
    } else if token == token_b {
        return balance_b;
    } else {
        // revert(TOKEN_NOT_REGISTERED);
        return 0x0000000000000000000000000000000000000000000000000000000000000000;
    }
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

    require(is_zero(balance_a) && is_zero(balance_b), Error::NONZERO_TOKEN_BALANCE);

    // delete _twoTokenPoolTokens[poolId];
    // delete poolBalances.sharedCash;
    // No delete methods for storage yet
    // need to implement it, as soon as we get support for that
}

#[storage(read, write)]
fn deregister_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let mut count = 0;

    while (count < tokens.len()) {
        let token = tokens.get(count).unwrap();
        let balance = storage.minimal_swap_info_pools_balances.get((pool_id, token, ));
        require(is_zero(balance), Error::NONZERO_TOKEN_BALANCE);

        // delete minimal_swap_info_pools_balances[poolId][token];
        // we need to delete the StorageMap in this case ^
        // but StorageMap does not have delete method on it
        // need to implemet it
        let token_removed = vec_remove_if_contains(pool_id, tokens.get(count).unwrap());
        if !token_removed {
            require(token_removed, Error::TOKEN_NOT_REGISTERED);
        }
        count = count + 1;
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
        count = count + 1;
    }

    return return_bool;
}

#[storage(write, read)]
fn deregister_general_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let mut i = 0;

    while (i < tokens.len()) {
        let token = tokens.get(i).unwrap();
        let current_balance = get_general_pool_balance_storage_map(pool_id, token);

        require(current_balance == 0x0000000000000000000000000000000000000000000000000000000000000000, Error::NONZERO_TOKEN_BALANCE);

        // pool_balances.remove(token);
        // no remove method on StorageMap
        // so assigning 0 to token
        let token = ~ContractId::from(token.into());
        set(pool_id, token, 0x0000000000000000000000000000000000000000000000000000000000000000);

        i += 1;
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
    let keyIndex = storage.indexes.get((pool_id, key, ));
    let mut pool_balances = storage.general_pools_balances.get(pool_id);

    // Equivalent to !contains(map, key)
    if keyIndex == 0 {
        let previousLength = pool_balances.length;
        let tmp = IERC20ToBytes32MapEntry {
            key: key,
            value: value,
        };
        storage.entries.insert((
            pool_id,
            previousLength,
        ), tmp);
        pool_balances.length = previousLength + 1;
        // The entry is stored at previousLength, but we add 1 to all indexes
        // and use 0 as a sentinel value
        storage.indexes.insert((pool_id, key, ), previousLength + 1);
        return true;
    } else {
        let tmp = IERC20ToBytes32MapEntry {
            key: storage.entries.get((
                pool_id,
                keyIndex - 1,
            )).key,
            value: value,
        };
        storage.entries.insert((
            pool_id,
            keyIndex - 1,
        ), tmp);
        return false;
    }
}

#[storage(read)]
fn get_value(pool_id: b256, key: ContractId) -> b256 {
    let index = storage.indexes.get((pool_id, key, ));
    require(index > 0, Error::OUT_OF_BOUNDS);
    return unchecked_value_at(pool_id, index - 1);
}

#[storage(read)]
fn unchecked_value_at(pool_id: b256, index: u64) -> b256 {
    return storage.entries.get((pool_id, index, )).value;
}

#[storage(read, write)]
fn register_two_token_pool_tokens(pool_id: b256, token_x: ContractId, token_y: ContractId) {
    require(token_x != token_y, Error::TOKEN_ALREADY_REGISTERED);
    let token_a: b256 = token_x.into();
    let token_b: b256 = token_y.into();
    require(token_a < token_b, Error::UNSORTED_TOKENS);

    let mut pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    require(pool_tokens.token_a == ~ContractId::from(ZERO_B256) && pool_tokens.token_a == ~ContractId::from(ZERO_B256), Error::TOKENS_ALREADY_SET);

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
            require(token_contains, Error::TOKEN_ALREADY_REGISTERED);
        }
        count = count + 1;
    }
}

#[storage(write, read)]
fn register_general_pool_tokens(pool_id: b256, tokens: Vec<ContractId>) {
    let pool_balances = storage.general_pools_balances.get(pool_id);

    let mut i = 0;
    while (i < tokens.len()) {
        let added = set(pool_id, tokens.get(i).unwrap(), 0x0000000000000000000000000000000000000000000000000000000000000000);
        require(added, Error::TOKEN_ALREADY_REGISTERED);
        // let token = tokens.get(i).unwrap();
        // if get_value(pool_id, token) == 0x0000000000000000000000000000000000000000000000000000000000000000 {
        //     revert(TOKEN_ALREADY_REGISTERED);
        // } else {
        //     // let token = token;
        //     set(pool_id, token, 0x0000000000000000000000000000000000000000000000000000000000000000);
        // }
        i = i + 1;
    }
}

// Converts a JoinPoolRequest into a PoolBalanceChange.
fn join_to_pool_balance_change(request: JoinPoolRequest) -> PoolBalanceChange {
    let mut assets = ~Vec::new();
    let mut maxAmountsIn = ~Vec::new();
    let mut amountsInOut = ~Vec::new();

    let mut count = 0;
    while count < 8 {
        if request.assets[count] == 0x0000000000000000000000000000000000000000000000000000000000000000 {
            break;
        }
        assets.push(~ContractId::from(request.assets[count]));
        maxAmountsIn.push(request.maxAmountsIn[count]);
        amountsInOut.push(request.userData.amountsInOut[count]);
        count = count + 1;
    }
    // assets.push(~ContractId::from(request.assets[0]));
    // assets.push(~ContractId::from(request.assets[1]));
    // maxAmountsIn.push(request.maxAmountsIn[0]);
    // maxAmountsIn.push(request.maxAmountsIn[1]);
    // amountsInOut.push(request.userData.amountsInOut[0]);
    // amountsInOut.push(request.userData.amountsInOut[1]);

    let userData = UserData {
        kind: request.userData.kind,
        amount: request.userData.amount,
        maxMinBPTAmount: request.userData.maxMinBPTAmount,
        bptAmountInOut: request.userData.bptAmountInOut,
        amountsInOut: amountsInOut,
    };
    let change = PoolBalanceChange {
        assets: assets,
        limits: maxAmountsIn,
        userData: userData,
        useInternalBalance: request.fromInternalBalance,
    };
    // let change = PoolBalanceChange {
    //     assets: request.assets,
    //     limits: request.maxAmountsIn,
    //     userData: request.userData,
    //     useInternalBalance: request.fromInternalBalance,
    // };
    return change;
}

// Converts a JoinPoolRequest into a PoolBalanceChange
fn exit_to_pool_balance_change(request: ExitPoolRequest) -> PoolBalanceChange {
    // solhint-disable-next-line no-inline-assembly
    let change = PoolBalanceChange {
        assets: request.assets,
        limits: request.minAmountsOut,
        userData: request.userData,
        useInternalBalance: request.toInternalBalance,
    };
    return change;
}

// Performs all `swaps`, calling swap hooks on the Pool contracts and updating their balances. Does not cause
// any transfer of tokens - instead it returns the net Vault token deltas: positive if the Vault should receive
// tokens, and negative if it should send them.
#[storage(write, read)]
fn swap_with_pool(
    swaps: Vec<BatchSwapStep>,
    assets: Vec<ContractId>,
    funds: FundManagement,
    kind: SwapKind,
) -> Vec<u64> {
    let mut assetDeltas: Vec<u64> = ~Vec::with_capacity(assets.len());

    // These store data about the previous swap here to implement multihop logic across swaps.
    let mut previousTokenCalculated: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);
    let mut previousAmountCalculated = 0;

    let mut count = 0;
    while count < swaps.len() {
        let mut batchSwapStep = swaps.get(count).unwrap();

        let mut withinBounds = false;
        if batchSwapStep.assetInIndex < assets.len()
            && batchSwapStep.assetOutIndex < assets.len()
        {
            withinBounds = true;
        }
        require(withinBounds, Error::OUT_OF_BOUNDS);

        let tokenIn = translate_to_ierc20(assets.get(batchSwapStep.assetInIndex).unwrap());
        let tokenOut = translate_to_ierc20(assets.get(batchSwapStep.assetOutIndex).unwrap());
        require(tokenIn != tokenOut, Error::CANNOT_SWAP_SAME_TOKEN);

        // Sentinel value for multihop logic
        if (batchSwapStep.amount == 0) {
            // When the amount given is zero, we use the calculated amount for the previous swap, as long as the
            // current swap's given token is the previous calculated token. This makes it possible to swap a
            // given amount of token A for token B, and then use the resulting token B amount to swap for token C.
            require(count > 0, Error::UNKNOWN_AMOUNT_IN_FIRST_SWAP);
            let usingPreviousToken = previousTokenCalculated == token_given(kind, tokenIn, tokenOut);
            require(usingPreviousToken, Error::MALCONSTRUCTED_MULTIHOP_SWP);
            batchSwapStep.amount = previousAmountCalculated;
        }

        // Initializing each struct field one-by-one uses less gas than setting all at once
        let poolRequest = SwapRequest {
            poolId: batchSwapStep.poolId,
            kind: kind,
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            amount: batchSwapStep.amount,
            userData: batchSwapStep.userData,
            from: funds.sender,
            to: funds.recipient, // todo The lastChangeBlock field is left uninitialized.
            lastChangeBlock: 0,
        };

        let mut amountIn = 0;
        let mut amountOut = 0;
        let (previousAmountCalculated, amountIn, amountOut) = swap_with_pool_hook(poolRequest);

        let previousTokenCalculated = token_calculated(kind, tokenIn, tokenOut);

        // Accumulate Vault deltas across swaps
        assetDeltas.push((assetDeltas.get(batchSwapStep.assetInIndex).unwrap() + amountIn));
        assetDeltas.swap(batchSwapStep.assetInIndex, assetDeltas.len() - 1);
        assetDeltas.pop();

        assetDeltas.push(assetDeltas.get(batchSwapStep.assetOutIndex).unwrap() - amountOut);
        assetDeltas.swap(batchSwapStep.assetInIndex, assetDeltas.len() - 1);
        assetDeltas.pop();
    }
    return assetDeltas;
}

// Performs a swap according to the parameters specified in `request`, calling the Pool's contract hook and
// updating the Pool's balance.
//
// Returns the amount of tokens going into or out of the Vault as a result of this swap, depending on the swap kind.
#[storage(write, read)]
fn swap_with_pool_hook(request: SwapRequest) -> (u64, u64, u64) {
    // Get the calculated amount from the Pool and update its balances
    let pool = get_pool_address(request.poolId);
    let specialization = get_pool_specialization(request.poolId);

    let amountCalculated = 0;
    if let PoolSpecialization::TWO_TOKEN = specialization {
        let amountCalculated = process_two_token_pool_swap_request(request, pool);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        let amountCalculated = process_minimal_swap_info_pool_swap_request(request, pool);
    } else {
        // PoolSpecialization::GENERAL
        let amountCalculated = process_general_pool_swap_request(request, pool);
    }

    let (amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);
    return (
        amountCalculated,
        amountIn,
        amountOut,
    );
}

#[storage(read)]
fn process_two_token_pool_swap_request(request: SwapRequest, pool: ContractId) -> u64 {
    // For gas efficiency reasons, this fn uses low-level knowledge of how Two Token Pool balances are
    // stored internally, instead of using getters and setters for all operations.
    let (tokenABalance, tokenBBalance, ) = get_two_token_pool_shared_balances(request.poolId, request.tokenIn, request.tokenOut);

    let pair_hash = storage.two_token_pool_tokens.get(request.poolId).balances;
    let mut poolBalances = storage.balances.get((
        request.poolId,
        pair_hash,
    ));

    // We have the two Pool balances, but we don't know which one is 'token in' or 'token out'.
    let mut tokenInBalance: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let mut tokenOutBalance: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

    // In Two Token Pools, token A has a smaller address than token B
    let tokenIn: b256 = request.tokenIn.into();
    let tokenOut: b256 = request.tokenOut.into();
    if tokenIn < tokenOut {
        // in is A, out is B
        tokenInBalance = tokenABalance;
        tokenOutBalance = tokenBBalance;
    } else {
        // in is B, out is A
        tokenOutBalance = tokenABalance;
        tokenInBalance = tokenBBalance;
    }

    // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    let (tokenInBalance, tokenOutBalance, amountCalculated) = call_minimal_swap_info_pool_on_swap_hook(request, pool, tokenInBalance, tokenOutBalance);

    // We check the token ordering again to create the new shared cash packed struct
    let tokenIn: b256 = request.tokenIn.into();
    let tokenOut: b256 = request.tokenOut.into();
    if tokenIn < tokenOut {
        poolBalances.shared_cash = to_shared_cash(tokenInBalance, tokenOutBalance); // in is A, out is B
    } else {
        poolBalances.shared_cash = to_shared_cash(tokenOutBalance, tokenInBalance); // in is B, out is A
    }

    return amountCalculated;
}

// Calls the onSwap hook for a Pool that implements IMinimalSwapInfoPool: both Minimal Swap Info and Two Token
// Pools do this.
fn call_minimal_swap_info_pool_on_swap_hook(
    request: SwapRequest,
    pool: ContractId,
    tokenInBalance: b256,
    tokenOutBalance: b256,
) -> (b256, b256, u64) {
    let mut request = request;
    let tokenInTotal = total(tokenInBalance);
    let tokenOutTotal = total(tokenOutBalance);
    request.lastChangeBlock = max(last_change_block(tokenInBalance), last_change_block(tokenOutBalance));

    // Perform the swap request callback, and compute the new balances for 'token in' and 'token out' after the swap
    // todo pool-utils/contracts/BaseMinimalSwapInfoPool
    // let amountCalculated = on_swap(pool, request, tokenInTotal, tokenOutTotal);
    let amountCalculated = 0;
    let (amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);

    let newTokenInBalance = increase_cash(tokenInBalance, amountIn);
    let newTokenOutBalance = decrease_cash(tokenOutBalance, amountOut);

    return (
        newTokenInBalance,
        newTokenOutBalance,
        amountCalculated,
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
        storage.minimal_swap_info_pools_balances.insert((pool_id, token, ), balances.get(count).unwrap());
        count = count + 1;
    }
}

#[storage(write, read)]
fn set_general_pool_balances(pool_id: b256, balances: Vec<b256>) {
    let mut count = 0;
    while (count < balances.len()) {
        let mut val = storage.entries.get((pool_id, count, ));
        val.value = balances.get(count).unwrap();
        count = count + 1;
    }
}

// this below function originally belong to fee contract
// Returns the protocol fee amount to charge for a flash loan of `amount`.
#[storage(read)]
fn calculate_flash_loan_fee_amount(amount: u64) -> u64 {
    // Fixed point multiplication introduces error: we round up, which means in certain scenarios the charged
    // percentage can be slightly higher than intended.
    let percentage = storage.flash_loan_fee_percentage;
    return mul_up(amount, percentage);
}

#[storage(read)] // Returns true if `token` is registered for `poolId`.
fn is_token_registered(poolId: b256, token: ContractId) -> bool {
    let specialization: PoolSpecialization = get_pool_specialization(poolId);
    if let PoolSpecialization::TWO_TOKEN = specialization {
        return is_two_token_pool_token_registered(poolId, token);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        return is_minimal_swap_info_pool_token_registered(poolId, token);
    } else {
        // PoolSpecialization::GENERAL
        return is_general_pool_token_registered(poolId, token);
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
    return storage.indexes.get((pool_id, token, )) != 0;
}

#[storage(read, write)]
fn perform_pool_management_operation(
    kind: PoolBalanceOpKind,
    poolId: b256,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    let specialization: PoolSpecialization = get_pool_specialization(poolId);
    if let PoolBalanceOpKind::WITHDRAW = kind {
        return withdraw_pool_balance(poolId, specialization, token, amount);
    } else if let PoolBalanceOpKind::DEPOSIT = kind {
        return deposit_pool_balance(poolId, specialization, token, amount);
    } else {
        // PoolBalanceOpKind::UPDATE
        return update_managed_balance(poolId, specialization, token, amount);
    }
}

// Moves `amount` tokens from a Pool's 'cash' to 'managed' balance, and transfers them to the caller.
// Returns the 'cash' and 'managed' balance deltas as a result of this call, which will be complementary.
#[storage(read, write)]
fn withdraw_pool_balance(
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    if let PoolSpecialization::TWO_TOKEN = specialization {
        update_two_token_pool_shared_balance_cash_to_managed(poolId, token, amount);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        update_minimal_swap_info_pool_balance_cash_to_managed(poolId, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        update_general_pool_balance_cash_to_managed(poolId, token, amount);
    }

    if (amount > 0) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        transfer_to_output(amount, token, sender);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(-amount);
    // managedDelta = int256(amount);
    return (amount, amount, );
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
    storage.minimal_swap_info_pools_balances.insert((pool_id, token, ), new_balance);

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
    let balance = storage.minimal_swap_info_pools_balances.get((pool_id, token, ));
    let token_registered = is_zero(balance) || vec_contains(storage.minimal_swap_info_pools_tokens.get(pool_id), token);

    if (!token_registered) {
        // PoolRegistry::_ensure_registered_pool(pool_id);
        revert(TOKEN_NOT_REGISTERED);
    }

    return balance;
}

#[storage(read, write)]
fn deposit_pool_balance(
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    if let PoolSpecialization::TWO_TOKEN = specialization {
        two_token_pool_managed_to_cash(poolId, token, amount);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        minimal_swap_info_pool_managed_to_cash(poolId, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        general_pool_managed_to_cash(poolId, token, amount);
    }

    if (amount > 0) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            },
            _ => {
                revert(0);
            },
        };
        transfer_to_output(amount, contract_id(), sender);
        // token.safeTransferFrom(msg.sender, address(this), amount);
    }

    // Since 'cash' and 'managed' are stored as uint112, `amount` is guaranteed to also fit in 112 bits. It will
    // therefore always fit in a 256 bit integer.
    // cashDelta = int256(amount);
    // managedDelta = int256(-amount);
    return (amount, amount, );
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
    storage.minimal_swap_info_pools_balances.insert((pool_id, token, ), new_balance);

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
    poolId: b256,
    specialization: PoolSpecialization,
    token: ContractId,
    amount: u64,
) -> (u64, u64) {
    let mut managedDelta = 0;
    if let PoolSpecialization::TWO_TOKEN = specialization {
        let managedDelta = set_two_token_pool_managed_balance(poolId, token, amount);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        let managedDelta = set_minimal_swap_info_pool_managed_balance(poolId, token, amount);
    } else {
        // PoolSpecialization::GENERAL
        let managedDelta = set_general_pool_managed_balance(poolId, token, amount);
    }

    // cashDelta = 0;
    return (0, managedDelta)
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
    storage.minimal_swap_info_pools_balances.insert((pool_id, token, ), new_balance);

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
        // return (0x0000000000000000000000000000000000000000000000000000000000000000, 0x0000000000000000000000000000000000000000000000000000000000000000);
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
    let tokenInBalance = get_minimal_swap_info_pool_balance(request.poolId, request.tokenIn);
    let tokenOutBalance = get_minimal_swap_info_pool_balance(request.poolId, request.tokenOut);

    // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    let (tokenInBalance, tokenOutBalance, amountCalculated) = call_minimal_swap_info_pool_on_swap_hook(request, pool, tokenInBalance, tokenOutBalance);
    storage.minimal_swap_info_pools_balances.insert((
        request.poolId,
        request.tokenIn,
    ), tokenInBalance);
    storage.minimal_swap_info_pools_balances.insert((
        request.poolId,
        request.tokenOut,
    ), tokenOutBalance);

    return amountCalculated;
}

#[storage(read, write)]
fn process_general_pool_swap_request(request: SwapRequest, pool: ContractId) -> u64 {
    let mut request = request;
    let mut tokenInBalance = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let mut tokenOutBalance = 0x0000000000000000000000000000000000000000000000000000000000000000;

    // We access both token indexes without checking existence, because we will do it manually immediately after.
    // todo need to check this
    let poolBalances = storage.general_pools_balances.get(request.poolId);
    let mut indexIn = storage.indexes.get((
        request.poolId,
        request.tokenIn,
    ));
    let mut indexOut = storage.indexes.get((
        request.poolId,
        request.tokenOut,
    ));

    if (indexIn == 0 || indexOut == 0) {
        // The tokens might not be registered because the Pool itself is not registered. We check this to provide a
        // more accurate revert reason.
        ensure_registered_pool(request.poolId);
        revert(TOKEN_NOT_REGISTERED);
    }

    // EnumerableMap stores indices *plus one* to use the zero index as a sentinel value - because these are valid,
    // we can undo this.
    indexIn = indexIn - 1;
    indexOut = indexOut - 1;

    let tokenAmount = poolBalances.length;
    let mut currentBalances = ~Vec::new();

    request.lastChangeBlock = 0;
    let mut count = 0;
    while count < tokenAmount {
        // Because the iteration is bounded by `tokenAmount`, and no tokens are registered or deregistered here, we
        // know `i` is a valid token index and can use `unchecked_valueAt` to save storage reads.
        let balance = unchecked_value_at(request.poolId, count);

        currentBalances.push(total(balance));
        request.lastChangeBlock = max(request.lastChangeBlock, last_change_block(balance));

        if (count == indexIn) {
            tokenInBalance = balance;
        } else if (count == indexOut) {
            tokenOutBalance = balance;
        }
    }

    // Perform the swap request callback and compute the new balances for 'token in' and 'token out' after the swap
    // todo this function belong to pool-utils/contracts/BaseGeneralPool
    // todo dummy value
    // let amountCalculated = on_swap(pool, request, currentBalances, indexIn, indexOut);
    let amountCalculated = 0;
    let (amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);
    tokenInBalance = increase_cash(tokenInBalance, amountIn);
    tokenOutBalance = decrease_cash(tokenOutBalance, amountOut);

    // Because no tokens were registered or deregistered between now or when we retrieved the indexes for
    // 'token in' and 'token out', we can use `unchecked_setAt` to save storage reads.
    unchecked_set_at(request.poolId, indexIn, tokenInBalance);
    unchecked_set_at(request.poolId, indexOut, tokenOutBalance);

    return amountCalculated;
}

#[storage(read, write)]
fn unchecked_set_at(pool_id: b256, index: u64, value: b256) {
    let mut entry = storage.entries.get((pool_id, index, ));
    entry.value = value;
    storage.entries.insert((pool_id, index, ), entry);
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
    poolId: b256,
    specialization: PoolSpecialization,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange,
) {
    // This fn uses a large number of stack variables (poolId, sender and recipient, balances, amounts, fees,
    // etc.), which leads to 'stack too deep' issues. It relies on private functions with seemingly arbitrary
    // interfaces to work around this limitation.
    require(change.assets.len() == change.limits.len(), Error::INPUT_LENGTH_MISMATCH);

    // We first check that the caller passed the Pool's registered tokens in the correct order, and retrieve the
    // current balance for each.
    let tokens = translate_to_ierc20_array(change.assets);
    // let balances = validate_tokens_and_get_balances(poolId, specialization, tokens);
    let (actualTokens, balances) = private_get_pool_tokens(poolId);

    // todo will comment when whole applicastion work, because tokens should be registered in the pool
    // require(actualTokens.len() == tokens.len(), Error::INPUT_LENGTH_MISMATCH);
    // require(actualTokens.len() > 0, Error::POOL_NO_TOKENS);
    let mut count = 0;
    while count < actualTokens.len() {
        require(actualTokens.get(count).unwrap() == tokens.get(count).unwrap(), Error::TOKENS_MISMATCH);
        count = count + 1;
    }

    // The bulk of the work is done here: the corresponding Pool hook is called, its final balances are computed,
    // assets are transferred, and fees are paid.
    let (finalBalances, amountsInOrOut, paidProtocolSwapFeeAmounts) = call_pool_balance_change(kind, poolId, sender, recipient, change, balances);

    // let mut finalBalances = ~Vec::new();
    // finalBalances.push(0x0000000000000000000000000000000000000000000000000000000000000143);
    // finalBalances.push(0x0000000000000000000000000000000000000000000000000000000000000020);
    // All that remains is storing the new Pool balances.
    if let PoolSpecialization::TWO_TOKEN = specialization {
        set_two_token_pool_cash_balances(poolId, tokens.get(0).unwrap(), finalBalances.get(0).unwrap(), tokens.get(1).unwrap(), finalBalances.get(1).unwrap());
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
    {
        set_minimal_swap_info_pool_balances(poolId, tokens, finalBalances);
    } else {
        // PoolSpecialization.GENERAL
        set_general_pool_balances(poolId, finalBalances);
    }

    // // Amounts in are positive, out are negative
    // let mut positive: bool = false;
    // if let PoolBalanceChangeKind::JOIN = kind {
    //     positive = true;
    // }
    // let balances_log = unsafe_cast_to_int256(amountsInOrOut, positive);
    // log(PoolBalanceChanged {
    //     pool_id: poolId,
    //     sender: sender,
    //     tokens: tokens,
    //     balances: balances_log,
    //     paid_protocol_swap_fee_amounts: paidProtocolSwapFeeAmounts,
    // });
}




// // Returns the total balance for `poolId`'s `expectedTokens`.
// // `expectedTokens` must exactly equal the token array returned by `getPoolTokens`: both arrays must have the same
// // length, elements and order. Additionally, the Pool must have at least one registered token.
// #[storage(read)]
// fn validate_tokens_and_get_balances(poolId: b256, expectedTokens: Vec<ContractId>) -> Vec<b256> {
//     let (actualTokens, balances) = private_get_pool_tokens(poolId);
//     require(actualTokens.len() == expectedTokens.len(), Error::INPUT_LENGTH_MISMATCH);
//     require(actualTokens.len() > 0, Error::POOL_NO_TOKENS);
//     let mut count = 0;
//     while count < actualTokens.len() {
//         require(actualTokens.get(count).unwrap() == expectedTokens.get(count).unwrap(), Error::TOKENS_MISMATCH);
//         count = count + 1;
//     }
//     // // todo dummy for now
//     // let balances: Vec<b256> = ~Vec::new();
//     return balances;
// }
// Calls the corresponding Pool hook to get the amounts in/out plus protocol fee amounts, and performs the
// associated token transfers and fee payments, returning the Pool's final balances.
#[storage(read, write)]
fn call_pool_balance_change(
    kind: PoolBalanceChangeKind,
    poolId: b256,
    sender: Address,
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
) -> (Vec<b256>, Vec<u64>, Vec<u64>) {
    let (totalBalances, lastChangeBlock) = totals_and_last_change_block(balances);

    // let get_protocol_swap_fee_percentage = MAX_PROTOCOL_SWAP_FEE_PERCENTAGE;
    let x = abi(ExternalInterface, poolId);
    let sender_contract: b256 = sender.into();
    let sender_contract_id = ~ContractId::from(sender_contract);
    let recipient_contract: b256 = recipient.into();
    let recipient_contract_id = ~ContractId::from(recipient_contract);
    if let PoolBalanceChangeKind::JOIN = kind {
        // todo dummy values for the build
        // let (amountsInOrOut, dueProtocolFeeAmounts) = x.on_join_pool(poolId, sender_contract_id, recipient_contract_id, totalBalances, lastChangeBlock, MAX_PROTOCOL_SWAP_FEE_PERCENTAGE, change.userData);
        let mut amountsInOrOut = ~Vec::new();
        amountsInOrOut.push(323);
        amountsInOrOut.push(31);
        let mut dueProtocolFeeAmounts = ~Vec::new();
        dueProtocolFeeAmounts.push(423);
        dueProtocolFeeAmounts.push(32);
        // require(balances.len() == amountsInOrOut.len() && amountsInOrOut.len() == dueProtocolFeeAmounts.len(), Error::INPUT_LENGTH_MISMATCH);
        // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
        // their participation.
        // let finalBalances = process_join_pool_transfers(sender, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
        let mut finalBalances = ~Vec::new();
        finalBalances.push(0x0000000000000000000000000000000000000000000000000000000000000143);
        finalBalances.push(0x0000000000000000000000000000000000000000000000000000000000000020);
        return (
            finalBalances,
            amountsInOrOut,
            dueProtocolFeeAmounts,
        );
    } else {
        // todo dummy values for the build
        // let (amountsInOrOut, dueProtocolFeeAmounts) = x.on_exit_pool(poolId, sender_contract_id, recipient_contract_id, totalBalances, lastChangeBlock, MAX_PROTOCOL_SWAP_FEE_PERCENTAGE, change.userData);
        let mut amountsInOrOut = ~Vec::new();
        amountsInOrOut.push(323);
        amountsInOrOut.push(31);
        let mut dueProtocolFeeAmounts = ~Vec::new();
        dueProtocolFeeAmounts.push(423);
        dueProtocolFeeAmounts.push(32);
        require(balances.len() == amountsInOrOut.len() && amountsInOrOut.len() == dueProtocolFeeAmounts.len(), Error::INPUT_LENGTH_MISMATCH);
        // The Vault ignores the `recipient` in joins and the `sender` in exits: it is up to the Pool to keep track of
        // their participation.
        let finalBalances = process_exit_pool_transfers(recipient, change, balances, amountsInOrOut, dueProtocolFeeAmounts);
        return (
            finalBalances,
            amountsInOrOut,
            dueProtocolFeeAmounts,
        );
    }
}

// Transfers `amountsIn` from `sender`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees.
// Returns the Pool's final balances, which are the current balances plus `amountsIn` minus accumulated protocol
// swap fees.
#[storage(read, write)]
fn process_join_pool_transfers(
    sender: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amountsIn: Vec<u64>,
    dueProtocolFeeAmounts: Vec<u64>,
) -> Vec<b256> {
    // We need to track how much of the received ETH was used and wrapped into WETH to return any excess.
    let mut wrappedEth = 0;

    let mut finalBalances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amountIn = amountsIn.get(count).unwrap();
        require(amountIn <= change.limits.get(count).unwrap(), Error::JOIN_ABOVE_MAX);

        // Receive assets from the sender - possibly from Internal Balance.
        let asset: ContractId = change.assets.get(count).unwrap();
        receive_asset(asset, amountIn, sender, change.useInternalBalance);
        if (is_eth(asset)) {
            wrappedEth = wrappedEth + amountIn;
        }

        let feeAmount = dueProtocolFeeAmounts.get(count).unwrap();
        // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        pay_fee_amount(translate_to_ierc20(asset), feeAmount);
        // Compute the new Pool balances. Note that the fee amount might be larger than `amountIn`,
        // resulting in an overall decrease of the Pool's balance for a token.
        // This lets us skip checked arithmetic
        if amountIn >= feeAmount {
            finalBalances.push(increase_cash(balances.get(count).unwrap(), amountIn - feeAmount));
        } else {
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
fn pay_fee_amount(token: ContractId, amount: u64) {
    if (amount > 0) {
        let address: b256 = contract_id().into();
        transfer_to_output(amount, token, ~Address::from(address));
    }
}

// Transfers `amountsOut` to `recipient`, checking that they are within their accepted limits, and pays
// accumulated protocol swap fees from the Pool.
//
// Returns the Pool's final balances, which are the current `balances` minus `amountsOut` and fees paid
// (`dueProtocolFeeAmounts`).
#[storage(read, write)]
fn process_exit_pool_transfers(
    recipient: Address,
    change: PoolBalanceChange,
    balances: Vec<b256>,
    amountsOut: Vec<u64>,
    dueProtocolFeeAmounts: Vec<u64>,
) -> Vec<b256> {
    let mut finalBalances = ~Vec::new();
    let mut count = 0;
    while count < change.assets.len() {
        let amountOut = amountsOut.get(count).unwrap();
        require(amountOut >= change.limits.get(count).unwrap(), Error::EXIT_BELOW_MIN);

        // Send tokens to the recipient - possibly to Internal Balance
        let asset = change.assets.get(count).unwrap();
        send_asset(asset, amountOut, recipient, change.useInternalBalance);

        let feeAmount = dueProtocolFeeAmounts.get(count).unwrap();
        pay_fee_amount(translate_to_ierc20(asset), feeAmount);

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

// // UserBalance
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

        log(ExternalBalanceTransfer {
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
    let currentBalance: u64 = get_internal_balance_private(account, token);
    let newBalance: u64 = amount + currentBalance;
    //todo- When INT256 is implemented
    set_internal_balance(account, token, newBalance, amount);
}

#[storage(read, write)]
fn receive_asset(
    asset: ContractId,
    amount: u64,
    sender: Address,
    fromInternalBalance: bool,
) {
    let mut amount = amount;
    if amount == 0 {
        return;
    }

    if is_eth(asset) {
        require(!fromInternalBalance, Error::INVALID_ETH_INTERNAL_BALANCE);

        // The ETH amount to receive is deposited into the WETH contract, which will in turn mint WETH for
        // the Vault at a 1:1 ratio.
        // A check for this condition is also introduced by the compiler, but this one provides a revert reason.
        // Note we're checking for the Vault's total balance, *not* ETH sent in this transaction.
        require(balance_of(BASE_ASSET_ID, contract_id()) >= amount, Error::INSUFFICIENT_ETH);
        force_transfer_to_contract(amount, BASE_ASSET_ID, contract_id());
    } else {
        let token = asset;

        if fromInternalBalance {
            // We take as many tokens from Internal Balance as possible: any remaining amounts will be transferred.
            let deductedBalance: u64 = decrease_internal_balance(sender, token, amount, true);
            // Because `deductedBalance` will be always the lesser of the current internal balance
            // and the amount to decrease, it is safe to perform unchecked arithmetic.
            amount = amount - deductedBalance;
        }

        if amount > 0 {
            force_transfer_to_contract(amount, token, contract_id());
        }
    }
}

// Sends `amount` of `asset` to `recipient`. If `toInternalBalance` is true, the asset is deposited as Internal
// Balance instead of being transferred.
//
// If `asset` is ETH, `toInternalBalance` must be false (as ETH cannot be held as internal balance), and the funds
// are instead sent directly after unwrapping WETH.
#[storage(read, write)]
fn send_asset(
    asset: ContractId,
    amount: u64,
    recipient: Address,
    toInternalBalance: bool,
) {
    if (amount == 0) {
        return;
    }

    if (is_eth(asset)) {
        // Sending ETH is not as involved as receiving it: the only special behavior is it cannot be
        // deposited to Internal Balance.
        require(!toInternalBalance, Error::INVALID_ETH_INTERNAL_BALANCE);
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
        if (toInternalBalance) {
            increase_internal_balance(recipient, token, amount);
        } else {
            transfer_to_output(amount, token, recipient);
        }
    }
}

// Decreases `account`'s Internal Balance for `token` by `amount`. If `allowPartial` is true, this function
// doesn't revert if `account` doesn't have enough balance, and sets it to zero and returns the deducted amount
// instead.
#[storage(read, write)]
fn decrease_internal_balance(
    account: Address,
    token: ContractId,
    amount: u64,
    allowPartial: bool,
) -> u64 {
    let currentBalance: u64 = get_internal_balance_private(account, token);
    require(allowPartial || (currentBalance >= amount), Error::INSUFFICIENT_INTERNAL_BALANCE);

    let deducted = currentBalance - amount;
    // By construction, `deducted` is lower or equal to `currentBalance`, so we don't need to use checked
    // arithmetic.
    let newBalance: u64 = currentBalance - deducted;

    // Todo When signed Integers are added
    set_internal_balance(account, token, newBalance, (deducted));
    return deducted;
}

// Sets `account`'s Internal Balance for `token` to `newBalance`.
//
// Emits an `InternalBalanceChanged` event. This event includes `delta`, which is the amount the balance increased
// (if positive) or decreased (if negative). To avoid reading the current balance in order to compute the delta,
// this function relies on the caller providing it directly.
// Todo When signed Integers are added
#[storage(read, write)]
fn set_internal_balance(
    account: Address,
    token: ContractId,
    newBalance: u64,
    delta: u64,
) {
    storage.internal_token_balance.insert((account, token, ), newBalance);
    log(InternalBalanceChanged {
        account: account,
        delta: delta,
        token: token,
    });
}

#[storage(read)]
fn get_internal_balance_private(account: Address, token: ContractId) -> u64 {
    return storage.internal_token_balance.get((account, token, ));
}

// PoolTokens
#[storage(read)] // Returns all of `pool_id`'s registered tokens, along with their raw balances.
fn private_get_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<b256>) {
    let specialization = storage.pool_specialization.get(pool_id);
    if let PoolSpecialization::TWO_TOKEN = specialization {
        return get_two_token_pool_tokens(pool_id);
    } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization
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

    if token_a == ZERO_ADDRESS || token_b == ZERO_ADDRESS {
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
    let mut tokens = ~Vec::new();
    let mut balances = ~Vec::new();

    let mut count = 0;
    while count < pool_tokens.len() {
        let token = pool_tokens.get(count).unwrap();
        tokens.push(token);
        balances.push(storage.minimal_swap_info_pools_balances.get((pool_id, token, )));
        count = count + 1;
    }

    // while count < pool_tokens.len() {
    //     tokens.push(ZERO_ADDRESS);
    //     count = count + 1;
    // }

    return (
        tokens,
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
        count = count + 1;
    }

    while count < pool_balances.length {
        balances.push(ZERO_B256);
        count = count + 1;
    }

    return (tokens, balances);
}

#[storage(read)]
fn unchecked_at(pool_id: b256, index: u64) -> (ContractId, b256) {
    let entry = storage.entries.get((pool_id, index, ));
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
fn ensure_registered_pool(poolId: b256) {
    require(storage.is_pool_registered.get(poolId), Error::INVALID_POOL_ID);
}
