library WeightedMath;

use std::{
    vec::Vec,
    option::Option,
    revert::require
};

use FixedPoint::*;
use math::{max, min, mul};
use BalancerErrors::*;


// A minimum normalized weight imposes a maximum weight ratio. We need this due to limitations in the
// implementation of the power fn, as these ratios are often exponents.
const _MIN_WEIGHT = 1;
// Having a minimum normalized weight imposes a limit on the maximum number of tokens;
// i.e., the largest possible pool is one where all tokens have exactly the minimum weight.
const _MAX_WEIGHTED_TOKENS = 100;

// Pool limits that arise from limitations in the fixed point power fn (and the imposed 1:100 maximum weight
// ratio).

// Swap limits: amounts swapped may not be larger than this percentage of total balance.
const _MAX_IN_RATIO = 3;
const _MAX_OUT_RATIO = 3;

// Invariant growth limit: non-proportional joins cannot cause the invariant to increase by more than this ratio.
const _MAX_INVARIANT_RATIO = 3;
// Invariant shrink limit: non-proportional exits cannot cause the invariant to decrease by less than this ratio.
const _MIN_INVARIANT_RATIO = 7;



// @dev Intermediate fn to avoid stack-too-deep errors.

fn _compute_join_exact_tokens_in_invariant_ratio(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    amountsIn: Vec<u64>,
    balanceRatiosWithFee: Vec<u64>,
    invariantRatioWithFees: u64,
    swapFeePercentage: u64
) -> u64 {
    let mut invariantRatio = ONE;

    let mut count = 0;
    while count < balances.len() {
        let mut amountInWithoutFee = 0;

        if balanceRatiosWithFee.get(count).unwrap() > invariantRatioWithFees {
            let nonTaxableAmount = mul_down(balances.get(count).unwrap(), sub(invariantRatioWithFees, ONE));
            let taxableAmount = sub(amountsIn.get(count).unwrap(), nonTaxableAmount);
            let swapFee = mul_up(taxableAmount, swapFeePercentage);

            amountInWithoutFee = add(nonTaxableAmount, sub(taxableAmount, swapFee));
        } else {
            amountInWithoutFee = amountsIn.get(count).unwrap();
        }
        let balanceRatio = div_down(add(balances.get(count).unwrap(), amountInWithoutFee), balances.get(count).unwrap());

        invariantRatio = mul_down(invariantRatio, pow_down(balanceRatio, normalizedWeights.get(count).unwrap()));
        
        count = count + 1;
        
    }
    invariantRatio
}



// @dev Intermediate fn to avoid stack-too-deep errors.
fn _compute_exit_exact_tokens_out_invariant_ratio(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    amountsOut: Vec<u64>,
    balanceRatiosWithoutFee: Vec<u64>,
    invariantRatioWithoutFees: u64,
    swapFeePercentage: u64
) -> u64 {
    let invariantRatio = ONE;

    let mut count = 0;
    while count < balances.len() {
        // Swap fees are typically charged on 'token in', but there is no 'token in' here, so we apply it to
        // 'token out'. This results in slightly larger price impact.

        let mut amountOutWithFee = 0;
        if (invariantRatioWithoutFees > balanceRatiosWithoutFee.get(count).unwrap()) {
            let nonTaxableAmount = mul_down(balances.get(count).unwrap(), complement(invariantRatioWithoutFees));
            let taxableAmount = sub(amountsOut.get(count).unwrap(), nonTaxableAmount);
            let taxableAmountPlusFees = div_up(taxableAmount, complement(swapFeePercentage));

            amountOutWithFee = add(nonTaxableAmount, taxableAmountPlusFees);
        } else {
            amountOutWithFee = amountsOut.get(count).unwrap();
        }

        let balanceRatio = div_down(sub(balances.get(count).unwrap(), amountOutWithFee), balances.get(count).unwrap());

        let invariantRatio = mul_down(invariantRatio, pow_down(balanceRatio, normalizedWeights.get(count).unwrap()));
        
        count = count + 1;
    }
    invariantRatio
}



// impl MyContract for Contract {

// About swap fees on joins and exits:
// Any join or exit that is not perfectly balanced (e.g. all single token joins or exits) is mathematically
// equivalent to a perfectly balanced join or exit followed by a series of swaps. Since these swaps would charge
// swap fees, it follows that (some) joins and exits should as well.
// On these operations, we split the token amounts in 'taxable' and 'non-taxable' portions, where the 'taxable' part
// is the one to which swap fees are applied.

// Invariant is used to collect protocol swap fees by comparing its value between two times.
// So we can round always to the same direction. It is also used to initiate the BPT amount
// and, because there is a minimum BPT, we round down the invariant.
pub fn _calculate_invariant(normalizedWeights: Vec<u64>, balances: Vec<u64>) -> u64 {       
    
    // invariant               _____                                                             //
    // wi = weight index i      | |      wi                                                      //
    // bi = balance index i     | |  bi ^   = i                                                  //
    // i = invariant                                                                             //
    

    let mut invariant = ONE;

    let mut count = 0;
    while count < balances.len() {
        invariant = mul_down(invariant, pow_down(balances.get(count).unwrap(), normalizedWeights.get(count).unwrap()));
        count = count + 1;
    } 

    require(invariant > 0, ZERO_INVARIANT);
    invariant
}

// Computes how many tokens can be taken out of a pool if `amountIn` are sent, given the
// current balances and weights.
pub fn _calc_out_given_in(
    balanceIn: u64,
    weightIn: u64,
    balanceOut: u64,
    weightOut: u64,
    amountIn: u64
) -> u64 {
    
    // outGivenIn                                                                                //
    // aO = amountOut                                                                            //
    // bO = balanceOut                                                                           //
    // bI = balanceIn              /      /            bI             \    (wI / wO) \           //
    // aI = amountIn    aO = bO * |  1 - | --------------------------  | ^            |          //
    // wI = weightIn               \      \       ( bI + aI )         /              /           //
    // wO = weightOut                                                                            //
    

    // Amount out, so we round down overall.

    // The multiplication rounds down, and the subtrahend (power) rounds up (so the base rounds up too).
    // Because bI / (bI + aI) <= 1, the exponent rounds down.

    // Cannot exceed maximum in ratio
    require(amountIn <= mul_down(balanceIn, _MAX_IN_RATIO), MAX_IN_RATIO); 

    let denominator = add(balanceIn, amountIn);
    let base = div_up(balanceIn, denominator);
    let exponent = div_down(weightIn, weightOut);
    let power = pow_up(base, exponent);

    mul_down(balanceOut, complement(power))
}

// Computes how many tokens must be sent to a pool in order to take `amountOut`, given the
// current balances and weights.
pub fn _calc_in_given_out(
    balanceIn: u64,
    weightIn: u64,
    balanceOut: u64,
    weightOut: u64,
    amountOut: u64
) -> u64 {
    
    // inGivenOut                                                                                //
    // aO = amountOut                                                                            //
    // bO = balanceOut                                                                           //
    // bI = balanceIn              /  /            bO             \    (wO / wI)      \          //
    // aI = amountIn    aI = bI * |  | --------------------------  | ^            - 1  |         //
    // wI = weightIn               \  \       ( bO - aO )         /                   /          //
    // wO = weightOut                                                                            //
    

    // Amount in, so we round up overall.

    // The multiplication rounds up, and the power rounds up (so the base rounds up too).
    // Because b0 / (b0 - a0) >= 1, the exponent rounds up.

    // Cannot exceed maximum out ratio
    require(amountOut <= mul_down(balanceOut, _MAX_OUT_RATIO), MAX_OUT_RATIO);

    let base = div_up(balanceOut, sub(balanceOut, amountOut));
    let exponent = div_up(weightOut, weightIn);
    let power = pow_up(base, exponent);

    // Because the base is larger than one (and the power rounds up), the power should always be larger than one, so
    // the following subtraction should never revert.
    let ratio = sub(power, ONE);

    mul_up(balanceIn, ratio)
}

pub fn _calc_bpt_out_given_exact_tokens_in(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    amountsIn: Vec<u64>,
    bptTotalSupply: u64,
    swapFeePercentage: u64
) -> u64 {
    // BPT out, so we round down overall.

    let mut balanceRatiosWithFee = ~Vec::new();

    let mut invariantRatioWithFees = 0;

    let mut count = 0;
    while count < balances.len() {
        balanceRatiosWithFee.push(div_down(add(balances.get(count).unwrap(), amountsIn.get(count).unwrap()), balances.get(count).unwrap()));
        invariantRatioWithFees = add(invariantRatioWithFees, mul_down(balanceRatiosWithFee.get(count).unwrap(), normalizedWeights.get(count).unwrap()));
        count = count + 1;
    }

    let invariantRatio = _compute_join_exact_tokens_in_invariant_ratio(
        balances,
        normalizedWeights,
        amountsIn,
        balanceRatiosWithFee,
        invariantRatioWithFees,
        swapFeePercentage
    );

    if invariantRatio > ONE {
        mul_down(bptTotalSupply, sub(invariantRatio, ONE))
    } 
    else {
        0
    }
}

pub fn _calc_token_in_given_exact_bpt_out(
    balance: u64,
    normalizedWeight: u64,
    bptAmountOut: u64,
    bptTotalSupply: u64,
    swapFeePercentage: u64
) -> u64 {
    // tokenInForExactBPTOut                                                                 //
    // a = amountIn                                                                          //
    // b = balance                      /  /    totalBPT + bptOut      \    (1 / w)       \  //
    // bptOut = bptAmountOut   a = b * |  | --------------------------  | ^          - 1  |  //
    // bpt = totalBPT                   \  \       totalBPT            /                  /  //
    // w = weight                                                                            //

    // Token in, so we round up overall.

    // Calculate the factor by which the invariant will increase after minting BPTAmountOut
    let invariantRatio = div_up(add(bptTotalSupply, bptAmountOut), bptTotalSupply);
    require(invariantRatio <= _MAX_INVARIANT_RATIO, MAX_OUT_BPT_FOR_TOKEN_IN);

    // Calculate by how much the token balance has to increase to match the invariantRatio
    let balanceRatio = pow_up(invariantRatio, div_up(ONE, normalizedWeight));

    let amountInWithoutFee = mul_up(balance, sub(balanceRatio, ONE));

    // We can now compute how much extra balance is being deposited and used in virtual swaps, and charge swap fees
    // accordingly.
    let taxableAmount = mul_up(amountInWithoutFee, complement(normalizedWeight));
    let nonTaxableAmount = sub(amountInWithoutFee, taxableAmount);

    let taxableAmountPlusFees = div_up(taxableAmount, complement(swapFeePercentage));

    add(nonTaxableAmount, taxableAmountPlusFees)
}

pub fn _calc_all_tokens_in_given_exact_bpt_out(
    balances: Vec<u64>,
    bptAmountOut: u64,
    totalBPT: u64
) -> Vec<u64> {
    
    // tokensInForExactBptOut                                                          //
    // (per token)                                                                     //
    // aI = amountIn                   /   bptOut   \                                  //
    // b = balance           aI = b * | ------------ |                                 //
    // bptOut = bptAmountOut           \  totalBPT  /                                  //
    // bpt = totalBPT                                                                  //

    // Tokens in, so we round up overall.
    let bptRatio = div_up(bptAmountOut, totalBPT);

    let mut amountsIn = ~Vec::new();

    let mut count = 0;
    while count < balances.len() {
        amountsIn.push(mul_up(balances.get(count).unwrap(), bptRatio));
        count = count + 1;
    }

    amountsIn
}

pub fn _calc_bpt_in_given_exact_tokens_out(
    balances: Vec<u64>,
    normalizedWeights: Vec<u64>,
    amountsOut: Vec<u64>,
    bptTotalSupply: u64,
    swapFeePercentage: u64
) -> u64 {
    // BPT in, so we round up overall.

    let mut balanceRatiosWithoutFee = ~Vec::new();
    let mut invariantRatioWithoutFees = 0;

    let mut count = 0;
    while count < balances.len() {
        balanceRatiosWithoutFee.push(div_up(sub(balances.get(count).unwrap(), amountsOut.get(count).unwrap()), balances.get(count).unwrap()));
        invariantRatioWithoutFees = mul_up(add(invariantRatioWithoutFees, balanceRatiosWithoutFee.get(count).unwrap()), normalizedWeights.get(count).unwrap());
        count = count + 1;
    }

    let invariantRatio = _compute_exit_exact_tokens_out_invariant_ratio(
        balances,
        normalizedWeights,
        amountsOut,
        balanceRatiosWithoutFee,
        invariantRatioWithoutFees,
        swapFeePercentage
    );

    mul_up(bptTotalSupply, complement(invariantRatio))
}

pub fn _calc_token_out_given_exact_bpt_in(
    balance: u64,
    normalizedWeight: u64,
    bptAmountIn: u64,
    bptTotalSupply: u64,
    swapFeePercentage: u64
) -> u64 {
    // exactBPTInForTokenOut                                                                //
    // a = amountOut                                                                        //
    // b = balance                     /      /    totalBPT - bptIn       \    (1 / w)  \   //
    // bptIn = bptAmountIn    a = b * |  1 - | --------------------------  | ^           |  //
    // bpt = totalBPT                  \      \       totalBPT            /             /   //
    // w = weight                                                                           //

    // Token out, so we round down overall. The multiplication rounds down, but the power rounds up (so the base
    // rounds up). Because (totalBPT - bptIn) / totalBPT <= 1, the exponent rounds down.

    // Calculate the factor by which the invariant will decrease after burning BPTAmountIn
    let invariantRatio = div_up(sub(bptTotalSupply, bptAmountIn), bptTotalSupply);
    require(invariantRatio >= _MIN_INVARIANT_RATIO, MIN_BPT_IN_FOR_TOKEN_OUT);

    // Calculate by how much the token balance has to decrease to match invariantRatio
    let balanceRatio = pow_up(invariantRatio, div_down(ONE, normalizedWeight));

    // Because of rounding up, balanceRatio can be greater than one. Using complement prevents reverts.
    let amountOutWithoutFee = mul_down(balance, complement(balanceRatio));

    // We can now compute how much excess balance is being withdrawn as a result of the virtual swaps, which result
    // in swap fees.

    // Swap fees are typically charged on 'token in', but there is no 'token in' here, so we apply it
    // to 'token out'. This results in slightly larger price impact. Fees are rounded up.
    let taxableAmount = mul_up(amountOutWithoutFee, complement(normalizedWeight));
    let nonTaxableAmount = sub(amountOutWithoutFee, taxableAmount);
    let taxableAmountMinusFees = mul_up(taxableAmount, complement(swapFeePercentage));

    add(nonTaxableAmount, taxableAmountMinusFees)
}

pub fn _calc_tokens_out_given_exact_bpt_in(
    balances: Vec<u64>,
    bptAmountIn: u64,
    totalBPT: u64
) -> Vec<u64> {
    // exactBPTInForTokensOut                                                                    //
    // (per token)                                                                               //
    // aO = amountOut                  /        bptIn         \                                  //
    // b = balance           a0 = b * | ---------------------  |                                 //
    // bptIn = bptAmountIn             \       totalBPT       /                                  //
    // bpt = totalBPT                                                                            //

    // Since we're computing an amount out, we round down overall. This means rounding down on both the
    // multiplication and division.

    let bptRatio = div_down(bptAmountIn, totalBPT);

    let mut amountsOut = ~Vec::new();
    
    let mut count = 0;
    while count < balances.len() {
        amountsOut.push(mul_down(balances.get(count).unwrap(), bptRatio));
        count = count + 1;
    }

    amountsOut
}

pub fn _calc_due_protocol_swap_fee_bpt_amount(
    totalSupply: u64,
    previousInvariant: u64,
    currentInvariant: u64,
    protocolSwapFeePercentage: u64
) -> u64 {
    // We round down to prevent issues in the Pool's accounting, even if it means paying slightly less in protocol
    // fees to the Vault.
    let growth = div_down(currentInvariant, previousInvariant);

    // Shortcut in case there was no growth when comparing the current against the previous invariant.
    // This shouldn't happen outside of rounding errors, but have this safeguard nonetheless to prevent the Pool
    // from entering a locked state in which joins and exits revert while computing accumulated swap fees.
    if growth <= ONE {
        // NOTE:- without adding return variable it's giving error.
        return 0;
    }

    // Assuming the Pool is balanced and token weights have not changed, a growth of the invariant translates into
    // proportional growth of all token balances. The protocol is due a percentage of that growth: more precisely,
    // it is due `k = protocol fee * (growth - 1) * balance / growth` for each token.
    // We compute the amount of BPT to mint for the protocol that would allow it to proportionally exit the Pool and
    // receive these balances. Note that the total BPT supply will increase when minting, so we need to account for
    // this in order to compute the percentage of Pool ownership the protocol will have.

    // The formula is:
    //
    // toMint = supply * k / (1 - k)

    // We compute protocol fee * (growth - 1) / growth, as we'll use that value twice.
    // There is no need to use SafeMath since we already checked growth is strictly greater than one.
    let k = div_down(mul_down(protocolSwapFeePercentage, growth - ONE), growth);

    let numerator = mul_down(totalSupply, k);
    let denominator = complement(k);

    if denominator == 0 {
        0
    } else {
        div_down(numerator, denominator)
    }

}

/**
    * @dev Calculate the amount of BPT which should be minted when adding a new token to the Pool.
    *
    * Note that normalizedWeight is set that it corresponds to the desired weight of this token *after* adding it.
    * i.e. For a two token 50:50 pool which we want to turn into a 33:33:33 pool, we use a normalized weight of 33%
    * @param totalSupply - the total supply of the Pool's BPT.
    * @param normalizedWeight - the normalized weight of the token to be added (normalized relative to final weights)
*/
pub fn _calc_bpt_out_add_token(totalSupply: u64, normalizedWeight: u64) -> u64 {
    // The amount of BPT which is equivalent to the token being added may be calculated by the growth in the
    // sum of the token weights, i.e. if we add a token which will make up 50% of the pool then we should receive
    // 50% of the new supply of BPT.
    //
    // The growth in the total weight of the pool can be easily calculated by:
    //
    // weightSumRatio = totalWeight / (totalWeight - newTokenWeight)
    //
    // As we're working with normalized weights `totalWeight` is equal to 1.

    let weightSumRatio = div_down(ONE, sub(ONE, normalizedWeight));

    // The amount of BPT to mint is then simply:
    //
    // toMint = totalSupply * (weightSumRatio - 1)

    mul_down(totalSupply, sub(weightSumRatio, ONE))
}

