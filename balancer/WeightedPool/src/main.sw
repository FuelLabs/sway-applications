contract;

dep data_structures;
dep interface;
dep errors;
dep utils;
dep events;

use interface::WeightedPool;
use errors::Error;
use data_structures::{
    DEFAULT_MINIMUM_BPT,
    ExitKind,
    JoinKind,
    MIN_SWAP_FEE_PERCENTAGE,
    TOTAL_SUPPLY,
    TOTAL_TOKENS,
    UNHANDLED_EXIT_KIND,
    UserData,
    RequestKind,
    UNHANDLED_JOIN_KIND,
};

use utils::{
    calc_bpt_out_given_exact_tokens_in,
    calc_due_protocol_swap_fee_bpt_amount,
    calc_token_in_given_exact_bpt_out,
    calculate_invariant,
    downscale_down_array,
    downscale_up_array,
    get_normalized_weights_private,
    scaling_factor,
    scaling_factors,
    upscale_array,
    calc_token_out_given_exact_bpt_in,
    calc_tokens_out_given_exact_bpt_in,
    calc_bpt_in_given_exact_tokens_out,
    join_all_tokens_in_for_exact_bptout,
};

use events::SwapFeePercentageChanged;

use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    constants::{BASE_ASSET_ID, ZERO_B256},
    context::{balance_of, call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    hash::keccak256,
    identity::Identity,
    logging::log,
    math::*,
    option::Option,
    reentrancy::is_reentrant,
    result::*,
    revert::{require, revert},
    storage::{StorageMap, get, store},
    token::{burn, force_transfer_to_contract, mint, transfer_to_output},
    vec::Vec,
};

// use vault::vault;

storage {
    last_post_join_exit_invariant: u64 = 10,
    misc_data: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    vault_contract_id: ContractId = ContractId {
        value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    },
    swap_fee_percentage: u64 = 0,
}

impl WeightedPool for Contract {
    // Vault hook for adding liquidity to a pool (including the first time, "initializing" the pool).
    // This fn can only be called from the Vault, from `joinPool`.
    #[storage(read, write)]fn on_join_pool(poolId: b256, sender: b256, recipient: b256, balances_array: [u64; 2], lastChangeBlock: u64, protocolSwapFeePercentage: u64, userData: UserData) { //-> (Vec<u64>, Vec<u64>) {
        let recipient = ~ContractId::from(recipient);
        let sender = ~ContractId::from(sender);

        let mut balances = ~Vec::new();
        let mut count = 0;
        while count < 2{
            balances.push(balances_array[count]);
            count = count + 1;
        }

        // _scaling_factors function exist in BaseWeightedPool contract
        let scalingFactors = scaling_factors();

        // if (totalSupply() == 0) {
        // _onInitializePool exist BaseWeightedPool cotract
        if TOTAL_SUPPLY == 0 {
            let(bptAmountOut, amountsIn) = on_initialize_pool(scalingFactors, userData);

            // On initialization, we lock _get_minimum_bpt() by minting it for the zero address. This BPT acts as a
            // minimum as it will never be burned, which reduces potential issues with rounding, and also prevents the
            // Pool from ever being fully drained.
            require(bptAmountOut >= DEFAULT_MINIMUM_BPT, Error::MINIMUM_BPT);
            // todo while minting the tokens contract is paniciing
            // mint_pool_tokens(~ContractId::from(ZERO_B256), DEFAULT_MINIMUM_BPT);
            // mint_pool_tokens(recipient, bptAmountOut - DEFAULT_MINIMUM_BPT);

            // amountsIn are amounts entering the Pool, so we round up.
            let amountsIn = downscale_up_array(amountsIn, scalingFactors);

            // return (amountsIn, ~Vec::with_capacity(balances.len()));
        } else {
            let balances = upscale_array(balances, scalingFactors);
            // _on_join_pool function exist in BaseWightedPool
            let(bptAmountOut, amountsIn) = on_join_pool_private(sender, balances, protocolSwapFeePercentage, scalingFactors, userData);

            // Note we no longer use `balances` after calling `_on_join_pool`, which may mutate it.

            // mint_pool_tokens(recipient, bptAmountOut);

            // amountsIn are amounts entering the Pool, so we round up.
            let amountsIn = downscale_up_array(amountsIn, scalingFactors);

            // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
            // return (amountsIn, ~Vec::with_capacity(balances.len()));
        }
    }

    // Vault hook for removing liquidity from a pool.
    // This fn can only be called from the Vault, from `exitPool`.
    #[storage(read, write)]fn on_exit_pool(poolId: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64, userData: UserData) -> (Vec<u64>, Vec<u64>) {
        let scalingFactors = scaling_factors();
        let balances = upscale_array(balances, scalingFactors);

        let(bptAmountIn, amountsOut) = on_exit_pool_private(sender, balances, protocolSwapFeePercentage, scalingFactors, userData);

        // Note we no longer use `balances` after calling `_on_exit_pool`, which may mutate it.
        burn_pool_tokens(sender, bptAmountIn);

        // amountsOut are amounts exiting the Pool, so we round down.
        let amountsOut = downscale_down_array(amountsOut, scalingFactors);

        // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
        (amountsOut, ~Vec::with_capacity(balances.len()))
    }

    // Set the swap fee percentage.
    // This is a permissioned fn, and disabled if the pool is paused. The swap fee must be within the
    // bounds set by MIN_SWAP_FEE_PERCENTAGE/MAX_SWAP_FEE_PERCENTAGE. Emits the SwapFeePercentageChanged event.
    #[storage(read, write)]fn set_swap_fee_percentage(swapFeePercentage: u64) {
        require(swapFeePercentage >= MIN_SWAP_FEE_PERCENTAGE, Error::MIN_SWAP_FEE_PERCENTAGE);
        require(swapFeePercentage <= MIN_SWAP_FEE_PERCENTAGE, Error::MAX_SWAP_FEE_PERCENTAGE);

        // storage.misc_data.insertUint(swapFeePercentage, SWAP_FEE_PERCENTAGE_OFFSET, 64);
        log(SwapFeePercentageChanged {
            swapFeePercentage: swapFeePercentage
        }
        );
    }

    // Query fns
    // "Dry run" `on_join_pool`.
    // Returns the amount of BPT that would be granted to `recipient` if the `on_join_pool` hook were called by the
    // Vault with the same arguments, along with the number of tokens `sender` would have to supply.
    //
    // This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
    // data, such as the protocol swap fee percentage and Pool balances.
    //
    // Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
    // explicitly use eth_call instead of eth_sendTransaction.
    #[storage(read, write)]fn query_join(poolId: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64, userData: UserData) -> (u64, Vec<u64>) {
        require(balances.len() == TOTAL_TOKENS, Error::INPUT_LENGTH_MISMATCH);

        let scalingFactors = scaling_factors();
        let balances = upscale_array(balances, scalingFactors);

        let(bptOut, amountsIn) = on_join_pool_private(sender, balances, protocolSwapFeePercentage, scalingFactors, userData);

        // if (msg.sender != address(this)) {
        //     query_action(
        //         poolId,
        //         sender,
        //         recipient,
        //         balances,
        //         lastChangeBlock,
        //         protocolSwapFeePercentage,
        //         userData,
        //     );
        // }
        // else {
        //     scalingFactors = scaling_factors();
        //     let balances = upscale_array(balances, scalingFactors);

        //     let(bptAmount, tokenAmounts) = on_join_pool(
        //         poolId,
        //         sender,
        //         recipient,
        //         balances,
        //         lastChangeBlock,
        //         protocolSwapFeePercentage,
        //         scalingFactors,
        //         userData
        //     );

        //     downscale_up_array(tokenAmounts, scalingFactors);

        //     // solhint-disable-next-line no-inline-assembly
        //     assembly {
        //         // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
        //         // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
        //         // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
        //         let size := mul(mload(tokenAmounts), 32)

        //         // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
        //         // will be at least one available slot due to how the memory scratch space works.
        //         // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
        //         let start := sub(tokenAmounts, 0x20)
        //         mstore(start, bptAmount)

        //         // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
        //         // We use the previous slot to `bptAmount`.
        //         mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
        //         start := sub(start, 0x04)

        //         // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
        //         // the `bptAmount`, the array 's length, and the error signature.
        //         revert(start, add(size, 68))
        //     }
        // }

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return(bptOut, amountsIn);
    }

    // "Dry run" `on_exit_pool`.
    // Returns the amount of BPT that would be burned from `sender` if the `on_exit_pool` hook were called by the
    // Vault with the same arguments, along with the number of tokens `recipient` would receive.
    //
    // This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
    // data, such as the protocol swap fee percentage and Pool balances.
    //
    // Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
    // explicitly use eth_call instead of eth_sendTransaction.
    #[storage(read, write)]fn query_exit(poolId: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, lastChangeBlock: u64, protocolSwapFeePercentage: u64, userData: UserData) -> (u64, Vec<u64>) {
        require(balances.len() == TOTAL_TOKENS, Error::INPUT_LENGTH_MISMATCH);

        let scalingFactors = scaling_factors();
        let balances = upscale_array(balances, scalingFactors);

        let(bptIn, amountsOut) = on_exit_pool_private(sender, balances, protocolSwapFeePercentage, scalingFactors, userData);

        // if (msg.sender != address(this)) {
        //     query_action(
        //         poolId,
        //         sender,
        //         recipient,
        //         balances,
        //         lastChangeBlock,
        //         protocolSwapFeePercentage,
        //         userData,
        //     );
        // }
        // else {
        //     scalingFactors: Vec<u64> = scaling_factors();
        //     let balances = upscale_array(balances, scalingFactors);

        //     (bptAmount: u64, uint256[] memory tokenAmounts) = on_exit_pool(
        //         poolId,
        //         sender,
        //         recipient,
        //         balances,
        //         lastChangeBlock,
        //         protocolSwapFeePercentage,
        //         scalingFactors,
        //         userData
        //     );

        //     downscale_down_array(tokenAmounts, scalingFactors);

        //     // solhint-disable-next-line no-inline-assembly
        //     assembly {
        //         // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
        //         // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
        //         // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
        //         let size := mul(mload(tokenAmounts), 32)

        //         // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
        //         // will be at least one available slot due to how the memory scratch space works.
        //         // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
        //         let start := sub(tokenAmounts, 0x20)
        //         mstore(start, bptAmount)

        //         // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
        //         // We use the previous slot to `bptAmount`.
        //         mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
        //         start := sub(start, 0x04)

        //         // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
        //         // the `bptAmount`, the array 's length, and the error signature.
        //         revert(start, add(size, 68))
        //     }
        // }

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return(bptIn, amountsOut);
    }

    /// Transfer coins to a target contract.
    fn force_transfer_coins(coins: u64, asset_id: ContractId, target: ContractId) {
        force_transfer_to_contract(coins, asset_id, target);
    }

    /// Transfer coins to a transaction output to be spent later.
    fn transfer_coins_to_output(coins: u64, asset_id: ContractId, recipient: Address) {
        transfer_to_output(coins, asset_id, recipient);
    }

    fn get_normalized_weights() -> Vec<u64> {
        return get_normalized_weights_private();
    }

    #[storage(read)]fn get_vault() -> ContractId {
        return storage.vault_contract_id;
    }

    // #[storage(read)]fn get_swap_fee_percentage() -> u64 {
    //     return storage.swap_fee_percentage;
    // }
}

#[storage(read)]fn get_swap_fee_percentage() -> u64 {
    return storage.swap_fee_percentage;
}

fn mint_pool_tokens(recipient: ContractId, amount: u64, ) {
    mint(amount);
    force_transfer_to_contract(amount, contract_id(), recipient);
}

fn burn_pool_tokens(sender: ContractId, bptAmountIn: u64) {
    force_transfer_to_contract(bptAmountIn, sender, contract_id());
    burn(bptAmountIn);
}

// Exit
#[storage(read, write)]fn on_exit_pool_private(sender: ContractId, balances: Vec<u64>, protocolSwapFeePercentage: u64, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // Exits are not disabled by default while the contract is paused, as some of them remain available to allow LPs
    // to safely exit the Pool in case of an emergency. Other exit kinds are disabled on a case-by-case basis in
    // their handlers.

    let normalizedWeights = get_normalized_weights_private();
    before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
    let(bptAmountIn, amountsOut) = do_exit( // sender,
    balances, normalizedWeights, scalingFactors, userData);
    after_join_exit(false, balances, amountsOut, normalizedWeights);

    (bptAmountIn, amountsOut)
}

// Dispatch code which decodes the provided userdata to perform the specified exit type.
// Inheriting contracts may override this fn to add additional exit types or extra conditions to allow
// or disallow exit under certain circumstances.
#[storage(read)]fn do_exit(balances: Vec<u64>, normalizedWeights: Vec<u64>, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    let mut res = (0, ~Vec::new());
    // todo abi.decode change exit kind 
    if let RequestKind::EXACT_TOKEN = userData.kind {
        res = exit_exact_bptin_for_token_out(balances, normalizedWeights, userData);
    } else if let RequestKind::EXACT_TOKENS_OUT = userData.kind {
        res = exit_exact_bptin_for_tokens_out(balances, userData);
    } else if let RequestKind::IN_FOR_EXACT_TOKENS_OUT = userData.kind {
        res = exit_bptin_for_exact_tokens_out(balances, normalizedWeights, scalingFactors, userData);
    } else {
        revert(UNHANDLED_EXIT_KIND);
    }
    res
}

#[storage(read)]
fn exit_bptin_for_exact_tokens_out(balances: Vec<u64>, normalizedWeights: Vec<u64>, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // This exit fn is disabled if the contract is paused.

    let amountsOut = userData.amountsInOut;
    let maxBPTAmountIn = userData.maxMinBPTAmount;
    require(amountsOut.len() == balances.len(), Error::INPUT_LENGTH_MISMATCH);
    upscale_array(amountsOut, scalingFactors);

    // todo: abi.decodes
    // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
    let bptAmountIn = calc_bpt_in_given_exact_tokens_out(
        balances,
        normalizedWeights,
        amountsOut,
        TOTAL_SUPPLY,
        get_swap_fee_percentage()
    );
    require(bptAmountIn <= maxBPTAmountIn, Error::BPT_IN_MAX_AMOUNT);

    (bptAmountIn, amountsOut)
    // (0, ~Vec::new())
}

fn exit_exact_bptin_for_tokens_out(balances: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // This exit fn is the only one that is not disabled if the contract is paused: it remains unrestricted
    // in an attempt to provide users with a mechanism to retrieve their tokens in case of an emergency.
    // This particular exit fn is the only one that remains available because it is the simplest one, and
    // therefore the one with the lowest likelihood of

    // todo abi.decode
    let bptAmountIn = userData.amount;
    // Note that there is no minimum amountOut parameter: this is handled by `IVault.exitPool`.

    let amountsOut = calc_tokens_out_given_exact_bpt_in(balances, bptAmountIn, TOTAL_SUPPLY);
    (bptAmountIn, amountsOut)
    // (0, ~Vec::new())
}

#[storage(read)]
fn exit_exact_bptin_for_token_out(balances: Vec<u64>, normalizedWeights: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // This exit fn is disabled if the contract is paused.

    // todo abi.decode
    let bptAmountIn = userData.amount;
    let tokenIndex = userData.maxMinBPTAmount;
    // Note that there is no minimum amountOut parameter: this is handled by `IVault.exitPool`.

    require(tokenIndex < balances.len(), Error::OUT_OF_BOUNDS);

    let amountOut = calc_token_out_given_exact_bpt_in(
        balances.get(tokenIndex).unwrap(),
        normalizedWeights.get(tokenIndex).unwrap(),
        bptAmountIn,
        TOTAL_SUPPLY,
        get_swap_fee_percentage()
    );

    // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
    // We exit in a single token, so we initialize amountsOut with zeros
    let mut amountsOut = ~Vec::new();
    let mut count = 0;
    while count < balances.len() {
        if count == tokenIndex {
            // And then assign the result to the selected token
            amountsOut.push(amountOut);
        }
        else {
            amountsOut.push(0);
        }
    }

    (bptAmountIn, amountsOut)
    // (0, ~Vec::new())
}

// Join
#[storage(read, write)]fn on_join_pool_private(sender: ContractId, balances: Vec<u64>, protocolSwapFeePercentage: u64, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // All joins are disabled while the contract is paused.

    let normalizedWeights = get_normalized_weights_private();

    before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
    let(bptAmountOut, amountsIn) = do_join(balances, normalizedWeights, scalingFactors, userData);
    after_join_exit(true, balances, amountsIn, normalizedWeights);

    (bptAmountOut, amountsIn)
}

#[storage(read)]fn join_exact_tokens_in_for_bptout(balances: Vec<u64>, normalizedWeights: Vec<u64>, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    // let(amountsIn, minBPTAmountOut) = userData.exactTokensInForBptOut();
    let amountsIn = userData.amountsInOut;
    let minBPTAmountOut = userData.maxMinBPTAmount;
    // ensure_input_length_match(balances.len(), balances.len());
    require(balances.len() == amountsIn.len(), Error::INPUT_LENGTH_MISMATCH);

    upscale_array(amountsIn, scalingFactors);

    // let x = abi(vault, storage.vault_contract_id);
    // let swap_fee_percentage = x.get_swap_fee_percentage();
    let bptAmountOut = calc_bpt_out_given_exact_tokens_in(balances, normalizedWeights, amountsIn, TOTAL_SUPPLY, get_swap_fee_percentage());

    require(bptAmountOut >= minBPTAmountOut, Error::BPT_OUT_MIN_AMOUNT);

    (bptAmountOut, amountsIn)
}

// Dispatch code which decodes the provided userdata to perform the specified join type.
// Inheriting contracts may override this pub fn to add additional join types or extra conditions to allow
// or disallow joins under certain circumstances.
#[storage(read)]fn do_join(balances: Vec<u64>, normalizedWeights: Vec<u64>, scalingFactors: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    // let kind = userData.joinKind();
    let mut res = (0, ~Vec::new());

    if let RequestKind::EXACT_TOKEN = userData.kind {
        res = join_exact_tokens_in_for_bptout(balances, normalizedWeights, scalingFactors, userData)
    } else if let RequestKind::EXACT_TOKENS_OUT = userData.kind {
        res = join_token_in_for_exact_bptout(balances, normalizedWeights, userData)
    } else if let RequestKind::IN_FOR_EXACT_TOKENS_OUT = userData.kind {
        res = join_all_tokens_in_for_exact_bptout(balances, userData)
    } else {
        revert(UNHANDLED_JOIN_KIND);
    }
    // (0, ~Vec::new())
    res
}

#[storage(read)]fn join_token_in_for_exact_bptout(balances: Vec<u64>, normalizedWeights: Vec<u64>, userData: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    let bptAmountOut = userData.amount;
    let tokenIndex = userData.maxMinBPTAmount;
    // let(bptAmountOut, tokenIndex) = userData.tokenInForExactBptOut();

    // Note that there is no maximum amountIn parameter: this is handled by `IVault.joinPool`.

    require(tokenIndex < balances.len(), Error::OUT_OF_BOUNDS);

    // let x = abi(vault, storage.vault_contract_id);
    // let swap_fee_percentage = x.get_swap_fee_percentage();
    let swap_fee_percentage = 10;
    let amountIn = calc_token_in_given_exact_bpt_out(balances.get(tokenIndex).unwrap(), normalizedWeights.get(tokenIndex).unwrap(), bptAmountOut, TOTAL_SUPPLY, get_swap_fee_percentage());

    // We join in a single token, so we initialize amountsIn with zeros

    let mut amountsIn: Vec<u64> = ~Vec::new();
    let mut count = 0;
    while count < balances.len() {
        if count == tokenIndex {
            // And then assign the result to the selected token
            amountsIn.push(amountIn);
        } else {
            amountsIn.push(0);
        }
    }

    return(bptAmountOut, amountsIn);
}

#[storage(read)]fn before_join_exit(preBalances: Vec<u64>, normalizedWeights: Vec<u64>, protocolSwapFeePercentage: u64) {
    // Before joins and exits, we measure the growth of the invariant compared to the invariant after the last join
    // or exit, which will have been caused by swap fees, and use it to mint BPT as protocol fees. This dilutes all
    // LPs, which means that new LPs will join the pool debt-free, and exiting LPs will pay any amounts due
    // before leaving.

    // We return immediately if the fee percentage is zero (to avoid unnecessary computation), or when the pool is
    // paused (to avoid complex computation during emergency withdrawals).
    let is_not_paused: bool = true;
    // import this function from TemporarilyPausable contract
    if ((protocolSwapFeePercentage == 0) || !is_not_paused) {
        return;
    }

    let preJoinExitInvariant = calculate_invariant(normalizedWeights, preBalances);

    let toMint = calc_due_protocol_swap_fee_bpt_amount(TOTAL_SUPPLY, storage.last_post_join_exit_invariant, preJoinExitInvariant, protocolSwapFeePercentage);

    // call this function from BasePool contract
    // payProtocolFees(toMint);
    mint_pool_tokens(storage.vault_contract_id, toMint);
}

#[storage(write)]fn on_initialize_pool(scalingFactors: Vec<u64>, userData: UserData // userData: JoinKind,
) -> (u64, Vec<u64>) {
    // It would be strange for the Pool to be paused before it is initialized, but for consistency we prevent
    // initialization in this case.

    // todo: abi.decode
    let mut flag = false;
    if let RequestKind::INIT = userData.kind {
        flag = true;
    }
    require(flag, Error::UNINITIALIZED);

    // todo: abi.decode
    let amountsIn = userData.amountsInOut;
    require(amountsIn.len() == scalingFactors.len(), Error::INPUT_LENGTH_MISMATCH);
    upscale_array(amountsIn, scalingFactors);

    let normalizedWeights: Vec<u64> = get_normalized_weights_private();
    let invariantAfterJoin = calculate_invariant(normalizedWeights, amountsIn);

    // Set the initial BPT to the value of the invariant times the number of tokens. This makes BPT supply more
    // consistent in Pools with similar compositions but different number of tokens.
    let bptAmountOut = invariantAfterJoin * amountsIn.len();

    after_join_exit(true, ~Vec::with_capacity(amountsIn.len()), amountsIn, normalizedWeights);
    (bptAmountOut, amountsIn)
}

#[storage(write)]fn after_join_exit(isJoin: bool, preBalances: Vec<u64>, balanceDeltas: Vec<u64>, normalizedWeights: Vec<u64>) {
    // After all joins and exits we store the post join/exit invariant in order to compute growth due to swap fees
    // in the next one.
    // let mut tmp = preBalances;
    let mut tmp = ~Vec::new();
    // Compute the post balances by adding or removing the deltas. Note that we're allowed to mutate preBalances.
    let mut count = 0;
    while count < preBalances.len() {
        // Cannot optimize calls with a fn selector: there are 2- and 3-argument versions of SafeMath.sub
        if isJoin {
            tmp.push(preBalances.get(count).unwrap() + balanceDeltas.get(count).unwrap());
        } else {
            tmp.push(preBalances.get(count).unwrap() - balanceDeltas.get(count).unwrap());
        }
    }

    // let postJoinExitInvariant = calculate_invariant(normalizedWeights, tmp);

    // storage.last_post_join_exit_invariant = postJoinExitInvariant;
}
