contract;

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
};


use FixedPoint::*;
use math::{mul};
use InputHelpers::{ensure_input_length_match};
// use BaseMinimalSwapInfoPool::*;
use WeightedMath::*;

use InvariantGrowthProtocolFees::InvariantGrowthProtocolFees;
use WeightedMath::{WeightedMath};

storage {
    InvariantGrowthProtocolFees_contract_id: ContractId = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0, 
}

// const TEMPORARILY_PAUSABLE_CONTRACT_ID = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0

struct SwapRequest {
    // IVault.SwapKind kind;
    tokenIn: ContractId,
    tokenOut: ContractId,
    amount: u64,
    // Misc data
    poolId: u8,
    lastChangeBlock: u64,
    from: Address,
    to: Address,
    userData: b256,
}


abi BaseWeightedPool {
    // fn get_invariant() -> u64;
    fn get_normalized_weights() -> Vec<u64>;
    fn _on_swap_given_in(
        swapRequest: SwapRequest,
        currentBalanceTokenIn: u64,
        currentBalanceTokenOut: u64
    ) -> u64;
    fn _on_swap_given_out(
        swapRequest: SwapRequest,
        currentBalanceTokenIn: u64,
        currentBalanceTokenOut: u64
    ) -> u64;
    fn _on_initialize_pool(
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>);
    fn _on_join_pool(
        sender: Address,
        balances: Vec<u64>,
        protocolSwapFeePercentage: u64,
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>);
    // fn _do_join(
    //     balances: Vec<u64>,
    //     normalizedWeights: Vec<u64>,
    //     scalingFactors: Vec<u64>,
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    // fn _join_exact_tokens_in_for_bptout(
    //     balances: Vec<u64>,
    //     normalizedWeights: Vec<u64>,
    //     scalingFactors: Vec<u64>,
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    // fn _join_token_in_for_exact_bptout(
    //     balances: Vec<u64>,
    //     normalizedWeights: Vec<u64>,
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    // fn _join_all_tokens_in_for_exact_bptout(
    //     balances: Vec<u64>, 
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    fn _on_exit_pool(
        sender: Address,
        balances: Vec<u64>,
        protocolSwapFeePercentage: u64,
        calingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>);
    // fn _do_exit(
    //     balances: Vec<u64>,
    //     normalizedWeights: Vec<u64>,
    //     scalingFactors: Vec<u64>,
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    fn _exit_exact_bptin_for_token_out(
        balances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>);
    // fn _exit_exact_bptin_for_tokens_out(
    //     balances: Vec<u64>, 
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);
    fn _exit_bptin_for_exact_tokens_out(
        balances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>);
    fn get_rate() -> u64;
}

const TOTAL_SUPPLY = 10000000;


fn _join_token_in_for_exact_bptout(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    userData: Vec<b256>
) -> (u64, Vec<u64>) {
    let(bptAmountOut, tokenIndex) = userData.tokenInForExactBptOut();
    // Note that there is no maximum amountIn parameter: this is handled by `IVault.joinPool`.

    require(tokenIndex < balances.len(), "OUT_OF_BOUNDS");

    let amountIn = _calc_token_in_given_exact_bpt_out(
        balances.get(tokenIndex).unwrap(),
        normalizedWeights.get(tokenIndex).unwrap(),
        bptAmountOut,
        TOTAL_SUPPLY,
        getSwapFeePercentage()
    );

    // We join in a single token, so we initialize amountsIn with zeros

    let mut amountsIn: Vec<u64> = ~Vec::new::<u64>();
    let mut count = 0;
    while count < balances.len() {
        if count == tokenIndex {
            // And then assign the result to the selected token
            amountsIn.push(amountIn);
        }
        else {
            amountsIn.push(0);
        }
    }

    (bptAmountOut, amountsIn)
}


fn _exit_exact_bptin_for_tokens_out(
    balances: Vec<u64>, 
    userData: Vec<b256>
) -> (u64, Vec<u64>)
{
    // This exit fn is the only one that is not disabled if the contract is paused: it remains unrestricted
    // in an attempt to provide users with a mechanism to retrieve their tokens in case of an emergency.
    // This particular exit fn is the only one that remains available because it is the simplest one, and
    // therefore the one with the lowest likelihood of 

    let bptAmountIn = userData.exactBptInForTokensOut();
    // Note that there is no minimum amountOut parameter: this is handled by `IVault.exitPool`.

    let amountsOut = _calc_tokens_out_given_exact_bpt_in(balances, bptAmountIn, TOTAL_SUPPLY);
    (bptAmountIn, amountsOut)
}


fn _join_all_tokens_in_for_exact_bptout(
    balances: Vec<u64>, 
    userData: Vec<b256>
) -> (u64, Vec<u64>) 
{
    let bptAmountOut = userData.allTokensInForExactBptOut();
    // Note that there is no maximum amountsIn parameter: this is handled by `IVault.joinPool`.

    let amountsIn = _calc_all_tokens_in_given_exact_bpt_out(
        balances,
        bptAmountOut,
        TOTAL_SUPPLY
    );

    (bptAmountOut, amountsIn)
}


/**
    * @dev Dispatch code which decodes the provided userdata to perform the specified join type.
    * Inheriting contracts may override this fn to add additional join types or extra conditions to allow
    * or disallow joins under certain circumstances.
    */
fn _do_join(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    scalingFactors: Vec<u64>,
    userData: Vec<b256>
) -> (u64, Vec<u64>) {
    let kind = userData.joinKind();

    if (kind == WeightedPoolUserData.JoinKind.EXACT_TOKENS_IN_FOR_BPT_OUT) {
        _join_exact_tokens_in_for_bptout(balances, normalizedWeights, scalingFactors, userData)
    } else if (kind == WeightedPoolUserData.JoinKind.TOKEN_IN_FOR_EXACT_BPT_OUT) {
        _join_token_in_for_exact_bptout(balances, normalizedWeights, userData)
    } else if (kind == WeightedPoolUserData.JoinKind.ALL_TOKENS_IN_FOR_EXACT_BPT_OUT) {
        _join_all_tokens_in_for_exact_bptout(balances, userData)
    } else {
        // _revert(UNHANDLED_JOIN_KIND);
        let tmp1 = 10;
        let tmp2 = ~Vec::new();
        return (tmp1, tmp2);
    }
}


/**
    * @dev Dispatch code which decodes the provided userdata to perform the specified exit type.
    * Inheriting contracts may override this fn to add additional exit types or extra conditions to allow
    * or disallow exit under certain circumstances.
    */

    // -> -> -> -> -> -> need to check what this is?
fn _do_exit(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    scalingFactors: Vec<u64>,
    userData: Vec<b256>
) -> (u64, Vec<u64>) {
    let kind = userData.exitKind();

    if (kind == WeightedPoolUserData.ExitKind.EXACT_BPT_IN_FOR_ONE_TOKEN_OUT) {
        return _exit_exact_bptin_for_token_out(balances, normalizedWeights, userData);
    } else if (kind == WeightedPoolUserData.ExitKind.EXACT_BPT_IN_FOR_TOKENS_OUT) {
        return _exit_exact_bptin_for_tokens_out(balances, userData);
    } else if (kind == WeightedPoolUserData.ExitKind.BPT_IN_FOR_EXACT_TOKENS_OUT) {
        return _exit_bptin_for_exact_tokens_out(balances, normalizedWeights, scalingFactors, userData);
    } else {
        // _revert(UNHANDLED_EXIT_KIND);
        let tmp1 = 10;
        let tmp2 = ~Vec::new();
        return (tmp1, tmp2);
    }
}


fn _join_exact_tokens_in_for_bptout(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    scalingFactors: Vec<u64>,
    userData: Vec<b256>
) -> (u64, Vec<u64>) {
    let(amountsIn, minBPTAmountOut) = userData.exactTokensInForBptOut();
    ensure_input_length_match(balances.len(), amountsIn.len());

    _upscale_array(amountsIn, scalingFactors);

    let bptAmountOut = _calc_bpt_out_given_exact_tokens_in(
        balances,
        normalizedWeights,
        amountsIn,
        // TOTAL_SUPPLY,
        TOTAL_SUPPLY,
        getSwapFeePercentage()
    );

    require(bptAmountOut >= minBPTAmountOut, "BPT_OUT_MIN_AMOUNT");

    (bptAmountOut, amountsIn)
}

/**
    * @dev Returns the current value of the invariant.
    */
fn get_invariant() -> u64 {
    let(_, balances, _) = getVault().getPoolTokens(getPoolId());

    // Since the Pool hooks always work with upscaled balances, we manually
    // upscale here for consistency
    _upscale_array(balances, _scaling_factors());
    // _get_normalized_weights function exist in WeightedPool
    let normalizedWeights: Vec<u64> = _get_normalized_weights();
    _calculate_invariant(normalizedWeights, balances)
}


impl BaseWeightedPool for Contract {
    /**
     * @dev Returns the normalized weight of `token`. Weights are fixed point numbers that sum to FixedPoint.ONE.
     */
    // fn _get_normalized_weight(token: ContractId) -> u64;

    /**
     * @dev Returns all normalized weights, in the same order as the Pool's tokens.
     */
    // fn _get_normalized_weights() internal view virtual returns (uint256[] memory);


    fn get_normalized_weights() -> Vec<u64> {
        _get_normalized_weights()
    }

    // Base Pool handlers

    // Swap

    fn _on_swap_given_in(
        swapRequest: SwapRequest,
        currentBalanceTokenIn: u64,
        currentBalanceTokenOut: u64
    ) -> u64 {
        // Swaps are disabled while the contract is paused.
        _calc_out_given_in(
            currentBalanceTokenIn,
            _get_normalized_weight(swapRequest.tokenIn),
            currentBalanceTokenOut,
            _get_normalized_weight(swapRequest.tokenOut),
            swapRequest.amount
        )
    }

    fn _on_swap_given_out(
        swapRequest: SwapRequest,
        currentBalanceTokenIn: u64,
        currentBalanceTokenOut: u64
    ) -> u64 {
        // Swaps are disabled while the contract is paused.

            _calc_in_given_out(
                currentBalanceTokenIn,
                _get_normalized_weight(swapRequest.tokenIn),
                currentBalanceTokenOut,
                _get_normalized_weight(swapRequest.tokenOut),
                swapRequest.amount
            )  
    }

    /**
     * @dev Called before any join or exit operation. Empty by default, but derived contracts may choose to add custom
     * behavior at these steps. This often has to do with protocol fee processing.
     */
    // fn _before_join_exit(
    //     preBalances: Vec<u64>,
    //     normalizedWeights: Vec<u64>,
    //     protocolSwapFeePercentage: u64
    // ) internal virtual {
    //     // solhint-disable-previous-line no-empty-blocks
    // }

    /**
     * @dev Called after any join or exit operation (including initialization). Empty by default, but derived contracts
     * may choose to add custom behavior at these steps. This often has to do with protocol fee processing.
     *
     * If isJoin is true, balanceDeltas are the amounts in: otherwise they are the amounts out.
     *
     * This fn is free to mutate the `preBalances` array.
     */
    // fn _after_join_exit(
    //     isJoin: bool,
    //     preBalances: Vec<u64>,
    //     balanceDeltas: Vec<u64>,
    //     normalizedWeights: Vec<u64>
    // ) internal virtual {
    //     // solhint-disable-previous-line no-empty-blocks
    // }

    // Initialize

    fn _on_initialize_pool(
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>) {
        // It would be strange for the Pool to be paused before it is initialized, but for consistency we prevent
        // initialization in this case.

        let kind = userData.joinKind();
        require(kind == WeightedPoolUserData.JoinKind.INIT, "UNINITIALIZED");

        let amountsIn: Vec<u64> = userData.initialAmountsIn();
        ensure_input_length_match(amountsIn.len(), scalingFactors.len());
        _upscale_array(amountsIn, scalingFactors);

        let normalizedWeights: Vec<u64> = _get_normalized_weights();
        let invariantAfterJoin = _calculate_invariant(normalizedWeights, amountsIn);

        // Set the initial BPT to the value of the invariant times the number of tokens. This makes BPT supply more
        // consistent in Pools with similar compositions but different number of tokens.
        let bptAmountOut = mul(invariantAfterJoin, amountsIn.len());

        _after_join_exit(true, ~Vec::with_capacity(amountsIn.len()), amountsIn, normalizedWeights);

        (bptAmountOut, amountsIn)
    }


    // Join

    fn _on_join_pool(
        sender: Address,
        balances: Vec<u64>,
        protocolSwapFeePercentage: u64,
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>) {
        // All joins are disabled while the contract is paused.

        let normalizedWeights: Vec<u64> = _get_normalized_weights();

        let x = abi(InvariantGrowthProtocolFees_contract_id, storage.InvariantGrowthProtocolFees_contract_id);
        x._before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
        let(bptAmountOut, amountsIn) = _do_join(
            // sender,
            balances,
            normalizedWeights,
            scalingFactors,
            userData
        );
        _after_join_exit(true, balances, amountsIn, normalizedWeights);

        (bptAmountOut, amountsIn)
    }


    // Exit
    fn _on_exit_pool(
        sender: Address,
        balances: Vec<u64>,
        protocolSwapFeePercentage: u64,
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>) {
        // Exits are not disabled by default while the contract is paused, as some of them remain available to allow LPs
        // to safely exit the Pool in case of an emergency. Other exit kinds are disabled on a case-by-case basis in
        // their handlers.

        let normalizedWeights = _get_normalized_weights();
        let x = abi(InvariantGrowthProtocolFees_contract_id, storage.InvariantGrowthProtocolFees_contract_id);
        x._before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
        let(bptAmountIn, amountsOut) = _do_exit(
            // sender,
            balances,
            normalizedWeights,
            scalingFactors,
            userData
        );
        _after_join_exit(false, balances, amountsOut, normalizedWeights);

        (bptAmountIn, amountsOut)
    }


    fn _exit_exact_bptin_for_token_out(
        balances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>) {
        // This exit fn is disabled if the contract is paused.

        let(bptAmountIn, tokenIndex) = userData.exactBptInForTokenOut();
        // Note that there is no minimum amountOut parameter: this is handled by `IVault.exitPool`.

        require(tokenIndex < balances.len(), "OUT_OF_BOUNDS");

        let amountOut = _calc_token_out_given_exact_bpt_in(
            balances.get(tokenIndex).unwrap(),
            normalizedWeights.get(tokenIndex).unwrap(),
            bptAmountIn,
            TOTAL_SUPPLY,
            getSwapFeePercentage()
        );

        // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
        // We exit in a single token, so we initialize amountsOut with zeros
        let mut amountsOut = ~Vec::new::<u64>();
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
    }

    fn _exit_bptin_for_exact_tokens_out(
        balances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        scalingFactors: Vec<u64>,
        userData: Vec<b256>
    ) -> (u64, Vec<u64>) {
        // This exit fn is disabled if the contract is paused.

        let(amountsOut, maxBPTAmountIn) = userData.bptInForExactTokensOut();
        ensure_input_length_match(amountsOut.len(), balances.len());
        _upscale_array(amountsOut, scalingFactors);

        // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
        let bptAmountIn = _calc_bpt_in_given_exact_tokens_out(
            balances,
            normalizedWeights,
            amountsOut,
            TOTAL_SUPPLY,
            getSwapFeePercentage()
        );
        require(bptAmountIn <= maxBPTAmountIn, "BPT_IN_MAX_AMOUNT");

        (bptAmountIn, amountsOut)
    }

    // Helpers

    /**
     * @dev This fn returns the appreciation of one BPT relative to the
     * underlying tokens. This starts at 1 when the pool is created and grows over time
     */
    fn get_rate() -> u64 {
        // The initial BPT supply is equal to the invariant times the number of tokens.
        div_down(mul(get_invariant(), _getTotalTokens()), TOTAL_SUPPLY)
    }
}
