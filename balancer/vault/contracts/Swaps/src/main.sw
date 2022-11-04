contract;

dep errors;
dep data_structures;
dep interface;
dep utils;

use errors::Error;
use data_structures::{
    TOKEN_NOT_REGISTERED,
    SwapKind,
    SingleSwap,
    FundManagement,
    SwapRequest,
    BatchSwapStep,
};
use interface::Swaps;
use utils::{
    token_given,
    token_calculated,
    handle_remaining_eth,
    get_amounts,
    get_pool_address,
    call_minimal_swap_info_pool_on_swap_hook,
};

use std::{
    vec::Vec,
    contract_id::ContractId,
    option::Option,
    address::Address,
    storage::{StorageMap,get, store},
    token::{force_transfer_to_contract,transfer_to_output},
    chain::auth::{AuthError, msg_sender},
    revert::{revert, require},
    math::*,
    identity::Identity,
    result::*,
    context::{call_frames::contract_id, msg_amount},
};

use BalanceAllocation::{
    last_change_block,
    increase_cash,
    decrease_cash,
    total,
};
use InputHelpers::ensure_input_length_match;
use AssetHelpers::{
    _is_eth,
    translate_to_ierc20,
};
use math::{
    add,
    max,
};

// use PoolRegistry::PoolRegistry;
// use MinimalSwapInfoPoolsBalance::MinimalSwapInfoPoolsBalance;
// use TwoTokenPoolsBalance::TwoTokenPoolsBalance;
// use UserBalance::UserBalance;

storage {
    pool_registry_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    minimal_swap_info_pools_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    two_pool_token_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    user_balance_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
}

// Implements the Vault's high-level swap functionality.
//
// Users can swap tokens with Pools by calling the `swap` and `batch_swap` functions. They need not trust the Pool
// contracts to do this: all security checks are made by the Vault.
//
// The `swap` fn executes a single swap, while `batch_swap` can perform multiple swaps in sequence.
// In each individual swap, tokens of one kind are sent from the sender to the Pool (this is the 'token in'),
// and tokens of another kind are sent from the Pool to the recipient in exchange (this is the 'token out').
// More complex swaps, such as one 'token in' to multiple tokens out can be achieved by batching together
// individual swaps.
impl Swaps for Contract {
    #[storage(read)]
    fn swap(
        singleSwap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64
    ) -> u64
    {
        // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
        // solhint-disable-next-line not-rely-on-time
        // require(block_timestamp() <= deadline, Error::SWAP_DEADLINE);

        // This revert reason is for consistency with `batch_swap`: an equivalent `swap` performed using that fn
        // would result in this error.
        require(singleSwap.amount > 0, Error::UNKNOWN_AMOUNT_IN_FIRST_SWAP);

        let tokenIn = translate_to_ierc20(singleSwap.assetIn);
        let tokenOut = translate_to_ierc20(singleSwap.assetOut);
        require(tokenIn != tokenOut, Error::CANNOT_SWAP_SAME_TOKEN);

        // Initializing each struct field one-by-one uses less gas than setting all at once.
        let poolRequest = SwapRequest{
            poolId: singleSwap.poolId,
            kind: singleSwap.kind,
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            amount: singleSwap.amount,
            userData: singleSwap.userData,
            from: funds.sender,
            to: funds.recipient,
            // todo The lastChangeBlock field is left uninitialized.
            lastChangeBlock: 0,
        };

        let amountIn = 0;
        let amountOut = 0;

        let(amountCalculated, amountIn, amountOut) = swap_with_pool_hook(poolRequest);
        if let SwapKind::GIVEN_IN = singleSwap.kind {
            require(amountOut >= limit, Error::SWAP_LIMIT);
        }
        else {
            require(amountIn <= limit, Error::SWAP_LIMIT);
        }

        // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        // x.receive_asset(singleSwap.assetIn, amountIn, funds.sender, funds.fromInternalBalance);
        // x.send_asset(singleSwap.assetOut, amountOut, funds.recipient, funds.toInternalBalance);

        // If the asset in is ETH, then `amountIn` ETH was wrapped into WETH.
        if _is_eth(singleSwap.assetIn) {
            handle_remaining_eth(amountIn);    
        }
        else {
            handle_remaining_eth(0);
        }
        return amountCalculated;
    }
    #[storage(read)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64
    ) -> Vec<u64>
    {
        // The deadline is timestamp-based: it should not be relied upon for sub-minute accuracy.
        // solhint-disable-next-line not-rely-on-time
        // require(block_timestamp() <= deadline, Error::SWAP_DEADLINE);

        ensure_input_length_match(assets.len(), limits.len());

        // Perform the swaps, updating the Pool token balances and computing the net Vault asset deltas.
        let assetDeltas = swap_with_pool(swaps, assets, funds, kind);

        // Process asset deltas, by either transferring assets from the sender (for positive deltas) or to the recipient
        // (for negative deltas).
        let mut wrappedEth = 0;
        let mut count = 0;
        while count < assets.len() {
            let asset = assets.get(count).unwrap();
            let delta = assetDeltas.get(count).unwrap();
            require(delta <= limits.get(count).unwrap(), Error::SWAP_LIMIT);

            // let x = abi(ProtocolFeesCollector, storage.protocol_fees_collector_contract_id);
            // if (delta > 0) {
            //     let toReceive = delta;
            //     x.receive_asset(asset, toReceive, funds.sender, funds.fromInternalBalance);

            //     if (_is_eth(asset)) {
            //         wrappedEth = wrappedEth+ toReceive;
            //     }
            // } else if (delta < 0) {
            //     // let toSend = -delta;
            //     let toSend = delta;
            //     x.send_asset(asset, toSend, funds.recipient, funds.toInternalBalance);
            // }
        }

        // Handle any used and remaining ETH.
        handle_remaining_eth(wrappedEth);

        return assetDeltas;
    }

    // This fn is not marked as `nonReentrant` because the underlying mechanism relies on reentrancy
    #[storage(read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement
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

        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address, _ => revert(0), 
        };
        let sender: b256 = sender.into();
        let this_contract = contract_id().into();
        if sender != this_contract {
            // We perform an external call to ourselves, forwarding the same calldata. In this call, the else clause of
            // the preceding if statement will be executed instead.

            // solhint-disable-next-line avoid-low-level-calls
            // let(success, _) = contract_id().call(msg.data);

            // // solhint-disable-next-line no-inline-assembly
            // assembly {
            //     // This call should always revert to decode the actual asset deltas from the revert reason
            //     switch success
            //         case 0 {
            //             // Note we are manually writing the memory slot 0. We can safely overwrite whatever is
            //             // stored there as we take full control of the execution and then immediately return.

            //             // We copy the first 4 bytes to check if it matches with the expected signature, otherwise
            //             // there was another revert reason and we should forward it.
            //             returndatacopy(0, 0, 0x04)
            //             let error := and(mload(0), 0xffffffff00000000000000000000000000000000000000000000000000000000)

            //             // If the first 4 bytes don't match with the expected signature, we forward the revert reason.
            //             if eq(eq(error, 0xfa61cc1200000000000000000000000000000000000000000000000000000000), 0) {
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
            //             // bytes, we start copying at address 0x20. We also get rid of the error signature, which takes
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
        } else {
            let deltas = swap_with_pool(swaps, assets, funds, kind);

            // // solhint-disable-next-line no-inline-assembly
            // assembly {
            //     // We will return a raw representation of the array in memory, which is composed of a 32 byte length,
            //     // followed by the 32 byte int256 values. Because revert expects a size in bytes, we multiply the array
            //     // length (stored at `deltas`) by 32.
            //     let size := mul(mload(deltas), 32)

            //     // We send one extra value for the error signature "QueryError(int256[])" which is 0xfa61cc12.
            //     // We store it in the previous slot to the `deltas` array. We know there will be at least one available
            //     // slot due to how the memory scratch space works.
            //     // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
            //     mstore(sub(deltas, 0x20), 0x00000000000000000000000000000000000000000000000000000000fa61cc12)
            //     let start := sub(deltas, 0x04)

            //     // When copying from `deltas` into returndata, we copy an additional 36 bytes to also return the array's
            //     // length and the error signature.
            //     revert(start, add(size, 36))
            // }
        }
        ~Vec::new()
    }
}


// Performs all `swaps`, calling swap hooks on the Pool contracts and updating their balances. Does not cause
// any transfer of tokens - instead it returns the net Vault token deltas: positive if the Vault should receive
// tokens, and negative if it should send them.
#[storage(read)]
fn swap_with_pool(
    swaps: Vec<BatchSwapStep>,
    assets: Vec<ContractId>,
    funds: FundManagement,
    kind: SwapKind,
) -> Vec<u64> {
    let mut assetDeltas = ~Vec::with_capacity(assets.len());

    // These store data about the previous swap here to implement multihop logic across swaps.
    let mut previousTokenCalculated: ContractId = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);
    let mut previousAmountCalculated = 0;

    let mut count = 0;
    while count < swaps.len() {
        let mut batchSwapStep = swaps.get(count).unwrap();

        let mut withinBounds = false;
        if batchSwapStep.assetInIndex < assets.len() && batchSwapStep.assetOutIndex < assets.len(){
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
            to: funds.recipient,
            // todo The lastChangeBlock field is left uninitialized.
            lastChangeBlock: 0,
        };

        let mut amountIn = 0;
        let mut amountOut = 0;
        let(previousAmountCalculated, amountIn, amountOut) = swap_with_pool_hook(poolRequest);

        let previousTokenCalculated = token_calculated(kind, tokenIn, tokenOut);

        // Accumulate Vault deltas across swaps
        assetDeltas.push(add(assetDeltas.get(batchSwapStep.assetInIndex).unwrap(), amountIn));
        assetDeltas.swap(batchSwapStep.assetInIndex, assetDeltas.len()-1);
        assetDeltas.pop();

        assetDeltas.push(assetDeltas.get(batchSwapStep.assetOutIndex).unwrap() -amountOut);
        assetDeltas.swap(batchSwapStep.assetInIndex, assetDeltas.len()-1);
        assetDeltas.pop();

    }
    return assetDeltas;
}

// Performs a swap according to the parameters specified in `request`, calling the Pool's contract hook and
// updating the Pool's balance.
//
// Returns the amount of tokens going into or out of the Vault as a result of this swap, depending on the swap kind.
#[storage(read)]
fn swap_with_pool_hook(request: SwapRequest)
    -> (u64, u64, u64)
{
    // Get the calculated amount from the Pool and update its balances
    let pool = get_pool_address(request.poolId);
    // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
    // let specialization = x.get_pool_specialization(request.poolId);

    // let amountCalculated = 0;
    // if let PoolSpecialization::TWO_TOKEN = specialization {
    //     let amountCalculated = process_two_token_pool_swap_request(request, ContractId);
    // } else if let PoolSpecialization::MINIMAL_SWAP_INFO = specialization {
    //     let amountCalculated = process_minimal_swap_info_pool_swap_request(request, ContractId);
    // } else {
    //     // PoolSpecialization::GENERAL
    //     let amountCalculated = process_general_pool_swap_request(request, ContractId);
    // }

    // let(amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);
    // return (amountCalculated, amountIn, amountOut);
    (0, 0, 0)
}

#[storage(read)]
fn process_two_token_pool_swap_request(request: SwapRequest, pool: ContractId)
    -> u64
{
    // For gas efficiency reasons, this fn uses low-level knowledge of how Two Token Pool balances are
    // stored internally, instead of using getters and setters for all operations.
    // let x = abi(TwoTokenPoolsBalance, storage.two_pool_token_balance_contract_id);
    // let(
    //     tokenABalance,
    //     tokenBBalance,
    //     poolBalances
    // ) = x.get_two_token_pool_shared_balances(request.poolId, request.tokenIn, request.tokenOut);

    // We have the two Pool balances, but we don't know which one is 'token in' or 'token out'.
    // let mut tokenInBalance: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    // let mut tokenOutBalance: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

    // // In Two Token Pools, token A has a smaller address than token B
    // let tokenIn: b256 = request.tokenIn.into();
    // let tokenOut: b256 = request.tokenOut.into();
    // if tokenIn < tokenOut {
    //     // in is A, out is B
    //     tokenInBalance = tokenABalance;
    //     tokenOutBalance = tokenBBalance;
    // } else {
    //     // in is B, out is A
    //     tokenOutBalance = tokenABalance;
    //     tokenInBalance = tokenBBalance;
    // }

    // // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    // let(tokenInBalance, tokenOutBalance, amountCalculated) = call_minimal_swap_info_pool_on_swap_hook(
    //     request,
    //     pool,
    //     tokenInBalance,
    //     tokenOutBalance
    // );

    // // We check the token ordering again to create the new shared cash packed struct
    // if poolBalances.sharedCash {
    //     poolBalances.sharedCash = BalanceAllocation.toSharedCash(tokenInBalance, tokenOutBalance); // in is A, out is B
    // }
    // else {
    //     poolBalances.sharedCash = BalanceAllocation.toSharedCash(tokenOutBalance, tokenInBalance); // in is B, out is A
    // }

    // return amountCalculated;
    0
}

#[storage(read)]
fn process_minimal_swap_info_pool_swap_request(
    request: SwapRequest,
    pool: ContractId 
) -> u64 {
    // let x = abi(MinimalSwapInfoPoolsBalance, storage.minimal_swap_info_pools_balance_contract_id);
    // let tokenInBalance = x.external_get_minimal_swap_info_pool_balance(request.poolId, request.tokenIn);
    // let tokenOutBalance = x.external_get_minimal_swap_info_pool_balance(request.poolId, request.tokenOut);

    // Perform the swap request and compute the new balances for 'token in' and 'token out' after the swap
    // let(tokenInBalance, tokenOutBalance, amountCalculated) = call_minimal_swap_info_pool_on_swap_hook(
    //     request,
    //     pool,
    //     tokenInBalance,
    //     tokenOutBalance
    // );
    
    // todo can't access the storage of other contract so created a function to update the balance
    // x.update_minimal_swap_info_pool_balances(request.poolId, request.tokenIn, tokenInBalance);
    // x.update_minimal_swap_info_pool_balances(request.poolId, request.tokenOut, tokenOutBalance);

    // return amountCalculated;
    0
}

#[storage(read)]
fn process_general_pool_swap_request(request: SwapRequest, pool: ContractId)
    -> u64
{   
    let mut request = request;
    let mut tokenInBalance = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let mut tokenOutBalance = 0x0000000000000000000000000000000000000000000000000000000000000000;

    // We access both token indexes without checking existence, because we will do it manually immediately after.
    // todo need to check this
    // EnumerableMap.IERC20ToBytes32Map storage poolBalances = _generalPoolsBalances[request.poolId];
    // let mut indexIn = poolBalances.unchecked_indexOf(request.tokenIn);
    // let mut indexOut = poolBalances.unchecked_indexOf(request.tokenOut);
// 
//     
    // if (indexIn == 0 || indexOut == 0) {
    //     // The tokens might not be registered because the Pool itself is not registered. We check this to provide a
    //     // more accurate revert reason.
    //     // let x = abi(PoolRegistry, storage.pool_registry_contract_id);
    //     // x.ensure_registered_pool(request.poolId);
    //     revert(TOKEN_NOT_REGISTERED);
    // }

    // // EnumerableMap stores indices *plus one* to use the zero index as a sentinel value - because these are valid,
    // // we can undo this.
    // indexIn = indexIn - 1;
    // indexOut = indexOut -1;

    // let tokenAmount = poolBalances.len();
    // let mut currentBalances = ~Vec::new();

    // request.lastChangeBlock = 0;
    // let mut count = 0;
    // while count < tokenAmount {
    //     // Because the iteration is bounded by `tokenAmount`, and no tokens are registered or deregistered here, we
    //     // know `i` is a valid token index and can use `unchecked_valueAt` to save storage reads.
    //     let balance = poolBalances.unchecked_valueAt(count);

    //     currentBalances.push(total(balance));
    //     request.lastChangeBlock = max(request.lastChangeBlock, last_change_block(balance));

    //     if (count == indexIn) {
    //         tokenInBalance = balance;
    //     } else if (count == indexOut) {
    //         tokenOutBalance = balance;
    //     }
    // }

    // // Perform the swap request callback and compute the new balances for 'token in' and 'token out' after the swap
    // // todo this function belong to pool-utils/contracts/BaseGeneralPool
    // let amountCalculated = on_swap(pool, request, currentBalances, indexIn, indexOut);
    // let(amountIn, amountOut) = get_amounts(request.kind, request.amount, amountCalculated);
    // tokenInBalance = increase_cash(tokenInBalance, amountIn);
    // tokenOutBalance = decrease_cash(tokenOutBalance, amountOut);

    // // Because no tokens were registered or deregistered between now or when we retrieved the indexes for
    // // 'token in' and 'token out', we can use `unchecked_setAt` to save storage reads.
    // poolBalances.unchecked_setAt(indexIn, tokenInBalance);
    // poolBalances.unchecked_setAt(indexOut, tokenOutBalance);

    // return amountCalculated;
    0
}