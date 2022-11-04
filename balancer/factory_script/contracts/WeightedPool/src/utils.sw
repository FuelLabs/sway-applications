library utils;

dep data_structures;
dep errors;

use errors::Error;
use data_structures::{
    FOUR,
    INVALID_TOKEN,
    MAX_INVARIANT_RATIO,
    MAX_POW_RELATIVE_ERROR,
    NORMALIZED_WEIGHT0,
    NORMALIZED_WEIGHT1,
    NORMALIZED_WEIGHT10,
    NORMALIZED_WEIGHT11,
    NORMALIZED_WEIGHT12,
    NORMALIZED_WEIGHT13,
    NORMALIZED_WEIGHT14,
    NORMALIZED_WEIGHT15,
    NORMALIZED_WEIGHT16,
    NORMALIZED_WEIGHT17,
    NORMALIZED_WEIGHT18,
    NORMALIZED_WEIGHT19,
    NORMALIZED_WEIGHT2,
    NORMALIZED_WEIGHT3,
    NORMALIZED_WEIGHT4,
    NORMALIZED_WEIGHT5,
    NORMALIZED_WEIGHT6,
    NORMALIZED_WEIGHT7,
    NORMALIZED_WEIGHT8,
    NORMALIZED_WEIGHT9,
    ONE,
    SCALING_FACTOR0,
    SCALING_FACTOR1,
    SCALING_FACTOR10,
    SCALING_FACTOR11,
    SCALING_FACTOR12,
    SCALING_FACTOR13,
    SCALING_FACTOR14,
    SCALING_FACTOR15,
    SCALING_FACTOR16,
    SCALING_FACTOR17,
    SCALING_FACTOR18,
    SCALING_FACTOR19,
    SCALING_FACTOR2,
    SCALING_FACTOR3,
    SCALING_FACTOR4,
    SCALING_FACTOR5,
    SCALING_FACTOR6,
    SCALING_FACTOR7,
    SCALING_FACTOR8,
    SCALING_FACTOR9,
    TOKEN0,
    TOKEN1,
    TOKEN10,
    TOKEN11,
    TOKEN12,
    TOKEN13,
    TOKEN14,
    TOKEN15,
    TOKEN16,
    TOKEN17,
    TOKEN18,
    TOKEN19,
    TOKEN2,
    TOKEN3,
    TOKEN4,
    TOKEN5,
    TOKEN6,
    TOKEN7,
    TOKEN8,
    TOKEN9,
    TOTAL_SUPPLY,
    TOTAL_TOKENS,
    TWO,
};

use std::{
    context::{balance_of, call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    math::*,
    option::Option,
    revert::{require, revert},
    token::{force_transfer_to_contract, mint, transfer_to_output},
    vec::Vec,
};

// Same as `_downscale_up`, but for an entire array. This pub fn does not return anything, but instead
// *mutates* the `amounts` array.
pub fn upscale_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = amounts;
    let mut count = 0;
    while count < TOTAL_TOKENS {
        tmp.push(div_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}

// Same as `_downscale_up`, but for an entire array. This fn does not return anything, but instead
// *mutates* the `amounts` array.
pub fn downscale_up_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = amounts;
    let mut count = 0;
    while count < TOTAL_TOKENS {
        tmp.push(div_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}

pub fn join_all_tokens_in_for_exact_bptout(balances: Vec<u64>, userData: Vec<b256>) -> (u64, Vec<u64>) {
    // todo: abi.decode
    // let bptAmountOut = userData.allTokensInForExactBptOut();
    let bptAmountOut = 0;
    // Note that there is no maximum amountsIn parameter: this is handled by `IVault.joinPool`.

    let amountsIn = calc_all_tokens_in_given_exact_bpt_out(balances, bptAmountOut, TOTAL_SUPPLY);

    return(bptAmountOut, amountsIn);
}

pub fn calc_all_tokens_in_given_exact_bpt_out(balances: Vec<u64>, bptAmountOut: u64, totalBPT: u64) -> Vec<u64> {
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

    return amountsIn;
}

pub fn div_up(a: u64, b: u64) -> u64 {
    require(b != 0, Error::ZERO_DIVISION);

    if (a == 0) {
        0
    } else {
        let aInflated = a * ONE;
        require(aInflated / a == ONE, Error::DIV_INTERNAL); // mul overflow

        // The traditional div_up formula is:
        // div_up(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // div_up(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, which we already tested for.

        return((aInflated - 1) / b) + 1;
    }
}

pub fn calc_token_in_given_exact_bpt_out(balance: u64, normalizedWeight: u64, bptAmountOut: u64, bptTotalSupply: u64, swapFeePercentage: u64) -> u64 {
    // tokenInForExactBPTOut                                                                 //
    // a = amountIn                                                                          //
    // b = balance                      /  /    totalBPT + bptOut      \    (1 / w)       \  //
    // bptOut = bptAmountOut   a = b * |  | --------------------------  | ^          - 1  |  //
    // bpt = totalBPT                   \  \       totalBPT            /                  /  //
    // w = weight                                                                            //

    // Token in, so we round up overall.

    // Calculate the factor by which the invariant will increase after minting BPTAmountOut
    let invariantRatio = div_up((bptTotalSupply + bptAmountOut), bptTotalSupply);
    require(invariantRatio <= MAX_INVARIANT_RATIO, Error::MAX_OUT_BPT_FOR_TOKEN_IN);

    // Calculate by how much the token balance has to increase to match the invariantRatio
    let balanceRatio = pow_up(invariantRatio, div_up(ONE, normalizedWeight));

    let amountInWithoutFee = mul_up(balance, (balanceRatio - ONE));

    // We can now compute how much extra balance is being deposited and used in virtual swaps, and charge swap fees
    // accordingly.
    let taxableAmount = mul_up(amountInWithoutFee, complement(normalizedWeight));
    let nonTaxableAmount = amountInWithoutFee - taxableAmount;

    let taxableAmountPlusFees = div_up(taxableAmount, complement(swapFeePercentage));

    return nonTaxableAmount + taxableAmountPlusFees;
}

// Returns x^y, assuming both are fixed point numbers, rounding up. The result is guaranteed to not be below
// the true value (that is, the error pub pub fn expected - actual is always negative).
pub fn pow_up(x: u64, y: u64) -> u64 {
    // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
    // and 80/20 Weighted Pools
    if (y == ONE) {
        return x;
    } else if (y == TWO) {
        return mul_up(x, x);
    } else if (y == FOUR) {
        let square = mul_up(x, x);
        return mul_up(square, square);
    } else {
        let raw = x.pow(y);
        let maxError = mul_up(raw, MAX_POW_RELATIVE_ERROR) + ONE;

        return raw + maxError;
    }
}

pub fn calc_bpt_out_given_exact_tokens_in(balances: Vec<u64>, normalizedWeights: Vec<u64>, amountsIn: Vec<u64>, bptTotalSupply: u64, swapFeePercentage: u64) -> u64 {
    // BPT out, so we round down overall.

    let mut balanceRatiosWithFee = ~Vec::new();

    let mut invariantRatioWithFees = 0;

    let mut count = 0;
    while count < balances.len() {
        balanceRatiosWithFee.push(div_down(balances.get(count).unwrap(), (amountsIn.get(count).unwrap()) + balances.get(count).unwrap()));
        invariantRatioWithFees = invariantRatioWithFees + mul_down(balanceRatiosWithFee.get(count).unwrap(), normalizedWeights.get(count).unwrap());
        count = count + 1;
    }

    let invariantRatio = compute_join_exact_tokens_in_invariant_ratio(balances, normalizedWeights, amountsIn, balanceRatiosWithFee, invariantRatioWithFees, swapFeePercentage);

    if invariantRatio > ONE {
        return mul_down(bptTotalSupply, (invariantRatio - ONE));
    } else {
        return 0;
    }
}

// Intermediate pub fn to avoid stack-too-deep errors.
pub fn compute_join_exact_tokens_in_invariant_ratio(balances: Vec<u64>, normalizedWeights: Vec<u64>, amountsIn: Vec<u64>, balanceRatiosWithFee: Vec<u64>, invariantRatioWithFees: u64, swapFeePercentage: u64) -> u64 {
    let mut invariantRatio = ONE;

    let mut count = 0;
    while count < balances.len() {
        let mut amountInWithoutFee = 0;

        if balanceRatiosWithFee.get(count).unwrap() > invariantRatioWithFees {
            let nonTaxableAmount = mul_down(balances.get(count).unwrap(), invariantRatioWithFees - ONE);
            let taxableAmount = amountsIn.get(count).unwrap() - nonTaxableAmount;
            let swapFee = mul_up(taxableAmount, swapFeePercentage);

            amountInWithoutFee = (taxableAmount - swapFee) + nonTaxableAmount;
        } else {
            amountInWithoutFee = amountsIn.get(count).unwrap();
        }
        let balanceRatio = div_down(amountInWithoutFee, (balances.get(count).unwrap()) + balances.get(count).unwrap());

        invariantRatio = mul_down(invariantRatio, pow_down(balanceRatio, normalizedWeights.get(count).unwrap()));

        count = count + 1;
    }
    invariantRatio
}

pub fn calc_due_protocol_swap_fee_bpt_amount(totalSupply: u64, previousInvariant: u64, currentInvariant: u64, protocolSwapFeePercentage: u64) -> u64 {
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

// Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
//
// Useful when computing the complement for values with some level of relative error, as it strips this error and
// prevents intermediate negative values.
pub fn complement(x: u64) -> u64 {
    if x < ONE {
        ONE - x
    } else {
        0
    }
}

pub fn div_down(a: u64, b: u64) -> u64 {
    require(b != 0, Error::ZERO_DIVISION);

    if (a == 0) {
        0
    } else {
        let aInflated = a * ONE;
        require(aInflated / a == ONE, Error::DIV_INTERNAL); // mul overflow

        aInflated / b
    }
}

pub fn scaling_factors() -> Vec<u64> {
    let totalTokens = get_total_tokens();
    let mut scalingFactors = ~Vec::new();

    scalingFactors.push(SCALING_FACTOR0);
    scalingFactors.push(SCALING_FACTOR1);
    if (totalTokens > 2) {
        scalingFactors.push(SCALING_FACTOR2);
    }
    if (totalTokens > 3) {
        scalingFactors.push(SCALING_FACTOR3);
    }
    if (totalTokens > 4) {
        scalingFactors.push(SCALING_FACTOR4);
    }
    if (totalTokens > 5) {
        scalingFactors.push(SCALING_FACTOR5);
    }
    if (totalTokens > 6) {
        scalingFactors.push(SCALING_FACTOR6);
    }
    if (totalTokens > 7) {
        scalingFactors.push(SCALING_FACTOR7);
    }
    if (totalTokens > 8) {
        scalingFactors.push(SCALING_FACTOR8);
    }
    if (totalTokens > 9) {
        scalingFactors.push(SCALING_FACTOR9);
    }
    if (totalTokens > 10) {
        scalingFactors.push(SCALING_FACTOR10);
    }
    if (totalTokens > 11) {
        scalingFactors.push(SCALING_FACTOR11);
    }
    if (totalTokens > 12) {
        scalingFactors.push(SCALING_FACTOR12);
    }
    if (totalTokens > 13) {
        scalingFactors.push(SCALING_FACTOR13);
    }
    if (totalTokens > 14) {
        scalingFactors.push(SCALING_FACTOR14);
    }
    if (totalTokens > 15) {
        scalingFactors.push(SCALING_FACTOR15);
    }
    if (totalTokens > 16) {
        scalingFactors.push(SCALING_FACTOR16);
    }
    if (totalTokens > 17) {
        scalingFactors.push(SCALING_FACTOR17);
    }
    if (totalTokens > 18) {
        scalingFactors.push(SCALING_FACTOR18);
    }
    if (totalTokens > 19) {
        scalingFactors.push(SCALING_FACTOR19);
    }

    let mut count = scalingFactors.len();
    while count < totalTokens {
        scalingFactors.push(0);
        count = count + 1;
    }

    return scalingFactors;
}

pub fn get_total_tokens() -> u64 {
    return TOTAL_TOKENS;
}

// // todo: this need to check again
// pub fn initial_amounts_in(userData: Vec<b256>) -> Vec<u64> {
//     // let(_, amountsIn) = abi.decode(userData, (JoinKind, Vec<u64>));
//     let amountsIn: Vec<u64> = ~Vec::new();
//     return amountsIn;
// }

// Returns x^y, assuming both are fixed point numbers, rounding down. The result is guaranteed to not be above
// the true value (that is, the error pub pub fn expected - actual is always positive).
pub fn pow_down(x: u64, y: u64) -> u64 {
    // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
    // and 80/20 Weighted Pools
    if (y == ONE) {
        x
    } else if (y == TWO) {
        mul_down(x, x)
    } else if (y == FOUR) {
        let square = mul_down(x, x);
        mul_down(square, square)
    } else {
        let raw = x.pow(y);
        let maxError = mul_up(raw, MAX_POW_RELATIVE_ERROR) + ONE;

        if (raw < maxError) {
            0
        } else {
            return raw - maxError;
        }
    }
}

pub fn mul_up(a: u64, b: u64) -> u64 {
    let product = a * b;
    require(a == 0 || product / a == b, Error::MUL_OVERFLOW);

    if product == 0 {
        0
    } else {
        // The traditional div_up formula is:
        // div_up(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // div_up(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, which we already tested for.
        let res: u64 = ((product - 1) / ONE) + 1;
        res
    }
}

pub fn get_normalized_weights_private() -> Vec<u64> {
    let totalTokens = get_total_tokens();
    let mut normalizedWeights = ~Vec::new();

    normalizedWeights.push(NORMALIZED_WEIGHT0);
    normalizedWeights.push(NORMALIZED_WEIGHT1);
    if (totalTokens > 2) {
        normalizedWeights.push(NORMALIZED_WEIGHT2);
    }
    if (totalTokens > 3) {
        normalizedWeights.push(NORMALIZED_WEIGHT3);
    }
    if (totalTokens > 4) {
        normalizedWeights.push(NORMALIZED_WEIGHT4);
    }
    if (totalTokens > 5) {
        normalizedWeights.push(NORMALIZED_WEIGHT5);
    }
    if (totalTokens > 6) {
        normalizedWeights.push(NORMALIZED_WEIGHT6);
    }
    if (totalTokens > 7) {
        normalizedWeights.push(NORMALIZED_WEIGHT7);
    }
    if (totalTokens > 8) {
        normalizedWeights.push(NORMALIZED_WEIGHT8);
    }
    if (totalTokens > 9) {
        normalizedWeights.push(NORMALIZED_WEIGHT9);
    }
    if (totalTokens > 11) {
        normalizedWeights.push(NORMALIZED_WEIGHT11);
    }
    if (totalTokens > 10) {
        normalizedWeights.push(NORMALIZED_WEIGHT10);
    }
    if (totalTokens > 12) {
        normalizedWeights.push(NORMALIZED_WEIGHT12);
    }
    if (totalTokens > 13) {
        normalizedWeights.push(NORMALIZED_WEIGHT13);
    }
    if (totalTokens > 14) {
        normalizedWeights.push(NORMALIZED_WEIGHT14);
    }
    if (totalTokens > 15) {
        normalizedWeights.push(NORMALIZED_WEIGHT15);
    }
    if (totalTokens > 16) {
        normalizedWeights.push(NORMALIZED_WEIGHT16);
    }
    if (totalTokens > 17) {
        normalizedWeights.push(NORMALIZED_WEIGHT17);
    }
    if (totalTokens > 18) {
        normalizedWeights.push(NORMALIZED_WEIGHT18);
    }
    if (totalTokens > 19) {
        normalizedWeights.push(NORMALIZED_WEIGHT19);
    }

    let mut count = totalTokens;
    while count < normalizedWeights.len() {
        normalizedWeights.push(0);
    }
    return normalizedWeights;
}

// Returns the scaling factor for one of the Pool's tokens. Reverts if token`
// is not a token registered by thePool.
pub fn scaling_factor(token: ContractId) -> u64 {
    if (token == TOKEN0) {
        return SCALING_FACTOR0;
    } else if token == TOKEN1 {
        return SCALING_FACTOR1;
    } else if token == TOKEN2 {
        return SCALING_FACTOR2;
    } else if token == TOKEN3 {
        return SCALING_FACTOR3;
    } else if token == TOKEN4 {
        return SCALING_FACTOR4;
    } else if token == TOKEN5 {
        return SCALING_FACTOR5;
    } else if token == TOKEN6 {
        return SCALING_FACTOR6;
    } else if token == TOKEN7 {
        return SCALING_FACTOR7;
    } else if token == TOKEN8 {
        return SCALING_FACTOR8;
    } else if token == TOKEN9 {
        return SCALING_FACTOR9;
    } else if token == TOKEN10 {
        return SCALING_FACTOR10;
    } else if token == TOKEN11 {
        return SCALING_FACTOR11;
    } else if token == TOKEN12 {
        return SCALING_FACTOR12;
    } else if token == TOKEN13 {
        return SCALING_FACTOR13;
    } else if token == TOKEN14 {
        return SCALING_FACTOR14;
    } else if token == TOKEN15 {
        return SCALING_FACTOR15;
    } else if token == TOKEN16 {
        return SCALING_FACTOR16;
    } else if token == TOKEN17 {
        return SCALING_FACTOR17;
    } else if token == TOKEN18 {
        return SCALING_FACTOR18;
    } else if token == TOKEN19 {
        return SCALING_FACTOR19;
    } else {
        revert(INVALID_TOKEN);
    }
}

// Invariant is used to collect protocol swap fees by comparing its value between two times.
// So we can round always to the same direction. It is also used to initiate the BPT amount
// and, because there is a minimum BPT, we round down the invariant.
pub fn calculate_invariant(normalizedWeights: Vec<u64>, balances: Vec<u64>) -> u64 {
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

    require(invariant > 0, Error::ZERO_INVARIANT);
    return invariant;
}

pub fn mul_down(a: u64, b: u64) -> u64 {
    let product = a * b;
    require(a == 0 || product / a == b, Error::MUL_OVERFLOW);

    product / ONE
}

// Same as `_downscaleDown`, but for an entire array. This fn does not return anything, but instead
// *mutates* the `amounts` array.
pub fn downscale_down_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = amounts;
    let mut count = 0;
    while count < TOTAL_TOKENS {
        tmp.push(div_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}
