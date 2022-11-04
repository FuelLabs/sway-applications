library utils;

dep data_structures;
dep errors;

use errors::Error;
use data_structures::UserData;

use std::{
    context::{
        balance_of,
        call_frames::{
            contract_id,
            msg_asset_id,
        },
        msg_amount,
    },
    contract_id::ContractId,
    math::*,
    option::Option,
    revert::{
        require,
        revert,
    },
    token::{
        force_transfer_to_contract,
        mint,
        transfer_to_output,
    },
    vec::Vec,
};


// This pub fn does not return anything, but instead *mutates* the `amounts` array.
pub fn upscale_array(amounts: Vec<u64>, scaling_factors: Vec<u64>) -> Vec<u64> {
    let mut new_amounts = ~Vec::new();

    let mut count = 0;
    while count < amounts.len() {
        new_amounts.insert(count, mul_down(amounts.get(count).unwrap(), scaling_factors.get(count).unwrap()));
        count += 1;
    }

    return new_amounts;
}

// This fn does not return anything, but instead *mutates* the `amounts` array.
pub fn downscale_up_array(amounts: Vec<u64>, scaling_factors: Vec<u64>) -> Vec<u64> {
    let mut new_amounts = ~Vec::new();
    let mut count = 0;
    while count < TOTAL_TOKENS {
        new_amounts.push(div_down(amounts.get(count).unwrap(), scaling_factors.get(count).unwrap()));
        count += 1;
    }
    while count < amounts.len() {
        new_amounts.push(amounts.get(count).unwrap());
        count += 1;
    }
    return new_amounts;
}

//  This fn does not return anything, but instead *mutates* the `amounts` array.
pub fn downscale_down_array(amounts: Vec<u64>, scaling_factors: Vec<u64>) -> Vec<u64> {
    let mut new_amounts = amounts;
    let mut count = 0;
    while count < TOTAL_TOKENS {
        new_amounts.push(div_down(amounts.get(count).unwrap(), scaling_factors.get(count).unwrap()));
        count += 1;
    }
    while count < amounts.len() {
        new_amounts.push(amounts.get(count).unwrap());
        count += 1;
    }
    return new_amounts;
}

/// WIEGHTED MATH FUNCTIONS FOR WIEGHTED POOL

pub fn calc_all_tokens_in_given_exact_bpt_out(
    balances: Vec<u64>,
    bpt_amount_out: u64,
    total_bpt: u64,
) -> Vec<u64> {
    // tokensInForExactBptOut                                                          //
    // (per token)                                                                     //
    // aI = amountIn                   /   bptOut   \                                  //
    // b = balance           aI = b * | ------------ |                                 //
    // bptOut = bpt_amount_out           \  total_bpt  /                                  //
    // bpt = total_bpt                                                                  //
    // Tokens in, so we round up overall.
    let bpt_ratio = div_up(bpt_amount_out, total_bpt);

    let mut amounts_in = ~Vec::new();

    let mut count = 0;
    while count < balances.len() {
        amounts_in.push(mul_up(balances.get(count).unwrap(), bpt_ratio));
        count += 1;
    }

    return amounts_in;
}

pub fn calc_token_in_given_exact_bpt_out(
    balance: u64,
    normalized_weight: u64,
    bpt_amount_out: u64,
    bpt_total_supply: u64,
    swap_fee_percentage: u64,
) -> u64 {
    // tokenInForExactBPTOut                                                                 //
    // a = amountIn                                                                          //
    // b = balance                      /  /    total_bpt + bptOut      \    (1 / w)       \  //
    // bptOut = bpt_amount_out   a = b * |  | --------------------------  | ^          - 1  |  //
    // bpt = total_bpt                   \  \       total_bpt            /                  /  //
    // w = weight                                                                            //
    // Token in, so we round up overall.
    // Calculate the factor by which the invariant will increase after minting BPTAmountOut
    let invariant_ratio = div_up((bpt_total_supply + bpt_amount_out), bpt_total_supply);
    require(invariant_ratio <= MAX_INVARIANT_RATIO, Error::MaxOutBptForTokenIn);

    // Calculate by how much the token balance has to increase to match the invariant_ratio
    let balance_ratio = pow_up(invariant_ratio, div_up(ONE, normalized_weight));

    let amount_in_without_fee = mul_up(balance, (balance_ratio - ONE));

    // We can now compute how much extra balance is being deposited and used in virtual swaps, and charge swap fees
    // accordingly.
    let taxable_amount = mul_up(amount_in_without_fee, complement(normalized_weight));
    let non_taxable_amount = amount_in_without_fee - taxable_amount;

    let taxable_amount_plus_fees = div_up(taxable_amount, complement(swap_fee_percentage));

    return non_taxable_amount + taxable_amount_plus_fees;
}



pub fn calc_bpt_out_given_exact_tokens_in(
    balances: Vec<u64>,
    normalized_weights: Vec<u64>,
    amounts_in: Vec<u64>,
    bpt_total_supply: u64,
    swap_fee_percentage: u64,
) -> u64 {
    // BPT out, so we round down overall.
    let mut balance_ratios_with_fee = ~Vec::new();

    let mut invariant_ratio_with_fees = 0;

    let mut count = 0;
    while count < balances.len() {
        balance_ratios_with_fee.push(div_down(balances.get(count).unwrap(), (amounts_in.get(count).unwrap()) + balances.get(count).unwrap()));
        invariant_ratio_with_fees = invariant_ratio_with_fees + mul_down(balance_ratios_with_fee.get(count).unwrap(), normalized_weights.get(count).unwrap());
        count += 1;
    }

    let invariant_ratio = compute_invariant_ratio(balances, normalized_weights, amounts_in, balance_ratios_with_fee, invariant_ratio_with_fees, swap_fee_percentage);

    if invariant_ratio > ONE {
        return mul_down(bpt_total_supply, (invariant_ratio - ONE));
    } else {
        return 0;
    }
}

// Intermediate pub fn to avoid stack-too-deep errors.
pub fn compute_invariant_ratio(
    balances: Vec<u64>,
    normalized_weights: Vec<u64>,
    amounts_in: Vec<u64>,
    balance_ratios_with_fee: Vec<u64>,
    invariant_ratio_with_fees: u64,
    swap_fee_percentage: u64,
) -> u64 {
    let mut invariant_ratio = ONE;

    let mut count = 0;
    while count < balances.len() {
        let mut amount_in_without_fee = 0;

        if balance_ratios_with_fee.get(count).unwrap() > invariant_ratio_with_fees
        {
            let non_taxable_amount = mul_down(balances.get(count).unwrap(), invariant_ratio_with_fees - ONE);
            let taxable_amount = amounts_in.get(count).unwrap() - non_taxable_amount;
            let swap_fee = mul_up(taxable_amount, swap_fee_percentage);

            amount_in_without_fee = (taxable_amount - swap_fee) + non_taxable_amount;
        } else {
            amount_in_without_fee = amounts_in.get(count).unwrap();
        }
        let balance_ratio = div_down(amount_in_without_fee, (balances.get(count).unwrap()) + balances.get(count).unwrap());

        invariant_ratio = mul_down(invariant_ratio, pow_down(balance_ratio, normalized_weights.get(count).unwrap()));

        count += 1;
    }
    invariant_ratio
}

pub fn calc_due_protocol_swap_fee_bpt_amount(
    total_supply: u64,
    previous_invariant: u64,
    current_invariant: u64,
    protocol_swap_fee_percentage: u64,
) -> u64 {
    // We round down to prevent issues in the Pool's accounting, even if it means paying slightly less in protocol
    // fees to the Vault.
    let growth = div_down(current_invariant, previous_invariant);

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
    let k = div_down(mul_down(protocol_swap_fee_percentage, growth - ONE), growth);

    let numerator = mul_down(total_supply, k);
    let denominator = complement(k);

    if denominator == 0 {
        0
    } else {
        div_down(numerator, denominator)
    }
}



pub fn scaling_factors() -> Vec<u64> {
    let total_tokens = get_total_tokens();
    let mut scaling_factors = ~Vec::new();

    scaling_factors.push(SCALING_FACTOR0);
    scaling_factors.push(SCALING_FACTOR1);
    if (total_tokens > 2) {
        scaling_factors.push(SCALING_FACTOR2);
    }
    if (total_tokens > 3) {
        scaling_factors.push(SCALING_FACTOR3);
    }
    if (total_tokens > 4) {
        scaling_factors.push(SCALING_FACTOR4);
    }
    if (total_tokens > 5) {
        scaling_factors.push(SCALING_FACTOR5);
    }
    if (total_tokens > 6) {
        scaling_factors.push(SCALING_FACTOR6);
    }
    if (total_tokens > 7) {
        scaling_factors.push(SCALING_FACTOR7);
    }
    if (total_tokens > 8) {
        scaling_factors.push(SCALING_FACTOR8);
    }
    if (total_tokens > 9) {
        scaling_factors.push(SCALING_FACTOR9);
    }
    if (total_tokens > 10) {
        scaling_factors.push(SCALING_FACTOR10);
    }
    if (total_tokens > 11) {
        scaling_factors.push(SCALING_FACTOR11);
    }
    if (total_tokens > 12) {
        scaling_factors.push(SCALING_FACTOR12);
    }
    if (total_tokens > 13) {
        scaling_factors.push(SCALING_FACTOR13);
    }
    if (total_tokens > 14) {
        scaling_factors.push(SCALING_FACTOR14);
    }
    if (total_tokens > 15) {
        scaling_factors.push(SCALING_FACTOR15);
    }
    if (total_tokens > 16) {
        scaling_factors.push(SCALING_FACTOR16);
    }
    if (total_tokens > 17) {
        scaling_factors.push(SCALING_FACTOR17);
    }
    if (total_tokens > 18) {
        scaling_factors.push(SCALING_FACTOR18);
    }
    if (total_tokens > 19) {
        scaling_factors.push(SCALING_FACTOR19);
    }

    // not needed because we will be making SCALING_FACTOR{N} const as 0 in data_structures.sw
    let mut count = scaling_factors.len();
    while count < total_tokens {
        scaling_factors.push(0);
        count += 1;
    }
    return scaling_factors;
}

pub fn get_total_tokens() -> u64 {
    return TOTAL_TOKENS;
}

pub fn get_normalized_weights_private() -> Vec<u64> {
    let total_tokens = get_total_tokens();
    let mut normalized_weights = ~Vec::new();

    normalized_weights.push(NORMALIZED_WEIGHT0);
    normalized_weights.push(NORMALIZED_WEIGHT1);
    if (total_tokens > 2) {
        normalized_weights.push(NORMALIZED_WEIGHT2);
    }
    if (total_tokens > 3) {
        normalized_weights.push(NORMALIZED_WEIGHT3);
    }
    if (total_tokens > 4) {
        normalized_weights.push(NORMALIZED_WEIGHT4);
    }
    if (total_tokens > 5) {
        normalized_weights.push(NORMALIZED_WEIGHT5);
    }
    if (total_tokens > 6) {
        normalized_weights.push(NORMALIZED_WEIGHT6);
    }
    if (total_tokens > 7) {
        normalized_weights.push(NORMALIZED_WEIGHT7);
    }
    if (total_tokens > 8) {
        normalized_weights.push(NORMALIZED_WEIGHT8);
    }
    if (total_tokens > 9) {
        normalized_weights.push(NORMALIZED_WEIGHT9);
    }
    if (total_tokens > 11) {
        normalized_weights.push(NORMALIZED_WEIGHT11);
    }
    if (total_tokens > 10) {
        normalized_weights.push(NORMALIZED_WEIGHT10);
    }
    if (total_tokens > 12) {
        normalized_weights.push(NORMALIZED_WEIGHT12);
    }
    if (total_tokens > 13) {
        normalized_weights.push(NORMALIZED_WEIGHT13);
    }
    if (total_tokens > 14) {
        normalized_weights.push(NORMALIZED_WEIGHT14);
    }
    if (total_tokens > 15) {
        normalized_weights.push(NORMALIZED_WEIGHT15);
    }
    if (total_tokens > 16) {
        normalized_weights.push(NORMALIZED_WEIGHT16);
    }
    if (total_tokens > 17) {
        normalized_weights.push(NORMALIZED_WEIGHT17);
    }
    if (total_tokens > 18) {
        normalized_weights.push(NORMALIZED_WEIGHT18);
    }
    if (total_tokens > 19) {
        normalized_weights.push(NORMALIZED_WEIGHT19);
    }

    let mut count = total_tokens;
    while count < normalized_weights.len() {
        normalized_weights.push(0);
        count += 1;
    }
    return normalized_weights;
}

// Returns the scaling factor for one of the Pool's tokens. Reverts if token`
// is not a token registered by thePool.
pub fn scaling_factor(token: ContractId) -> u64 {
    let token: b256 = token.into();
    let mut scaling_factor = 0;
    if (token == TOKEN0) {
        scaling_factor = SCALING_FACTOR0;
    } else if token == TOKEN1 {
        scaling_factor = SCALING_FACTOR1;
    } else if token == TOKEN2 {
        scaling_factor = SCALING_FACTOR2;
    } else if token == TOKEN3 {
        scaling_factor = SCALING_FACTOR3;
    } else if token == TOKEN4 {
        scaling_factor = SCALING_FACTOR4;
    } else if token == TOKEN5 {
        scaling_factor = SCALING_FACTOR5;
    } else if token == TOKEN6 {
        scaling_factor = SCALING_FACTOR6;
    } else if token == TOKEN7 {
        scaling_factor = SCALING_FACTOR7;
    } else if token == TOKEN8 {
        scaling_factor = SCALING_FACTOR8;
    } else if token == TOKEN9 {
        scaling_factor = SCALING_FACTOR9;
    } else if token == TOKEN10 {
        scaling_factor = SCALING_FACTOR10;
    } else if token == TOKEN11 {
        scaling_factor = SCALING_FACTOR11;
    } else if token == TOKEN12 {
        scaling_factor = SCALING_FACTOR12;
    } else if token == TOKEN13 {
        scaling_factor = SCALING_FACTOR13;
    } else if token == TOKEN14 {
        scaling_factor = SCALING_FACTOR14;
    } else if token == TOKEN15 {
        scaling_factor = SCALING_FACTOR15;
    } else if token == TOKEN16 {
        scaling_factor = SCALING_FACTOR16;
    } else if token == TOKEN17 {
        scaling_factor = SCALING_FACTOR17;
    } else if token == TOKEN18 {
        scaling_factor = SCALING_FACTOR18;
    } else if token == TOKEN19 {
        scaling_factor = SCALING_FACTOR19;
    } else {
        revert(INVALID_TOKEN);
    }
    return scaling_factor;
}

// Invariant is used to collect protocol swap fees by comparing its value between two times.
// So we can round always to the same direction. It is also used to initiate the BPT amount
// and, because there is a minimum BPT, we round down the invariant.
pub fn calculate_invariant(normalized_weights: Vec<u64>, balances: Vec<u64>) -> u64 {
    // invariant               _____                                                             //
    // wi = weight index i      | |      wi                                                      //
    // bi = balance index i     | |  bi ^   = i                                                  //
    // i = invariant                                                                             //
    let mut invariant = 1;

    let mut count = 0;
    while count < balances.len() {
        invariant = invariant * ((balances.get(count).unwrap() * normalized_weights.get(count).unwrap()));
        count += 1;
    }

    require(invariant > 0, Error::ZeroInvariant);
    return invariant;
}

pub fn calc_bpt_in_given_exact_tokens_out(
    balances: Vec<u64>,
    normalized_weights: Vec<u64>,
    amounts_out: Vec<u64>,
    bpt_total_supply: u64,
    swap_fee_percentage: u64,
) -> u64 {
    // BPT in, so we round up overall.
    let mut balance_ratios_without_fee = ~Vec::new();
    let mut invariant_ratio_without_fees = 0;

    let mut count = 0;
    while count < balances.len() {
        balance_ratios_without_fee.push(div_up((balances.get(count).unwrap() - amounts_out.get(count).unwrap()), balances.get(count).unwrap()));
        invariant_ratio_without_fees = mul_up((invariant_ratio_without_fees + balance_ratios_without_fee.get(count).unwrap()), normalized_weights.get(count).unwrap());
        count += 1;
    }

    let invariant_ratio = compute_exit_exact_tokens_out_invariant_ratio(balances, normalized_weights, amounts_out, balance_ratios_without_fee, invariant_ratio_without_fees, swap_fee_percentage);

    mul_up(bpt_total_supply, complement(invariant_ratio))
}
// Intermediate fn to avoid stack-too-deep errors.
fn compute_exit_exact_tokens_out_invariant_ratio(
    balances: Vec<u64>,
    normalized_weights: Vec<u64>,
    amounts_out: Vec<u64>,
    balance_ratios_without_fee: Vec<u64>,
    invariant_ratio_without_fees: u64,
    swap_fee_percentage: u64,
) -> u64 {
    let invariant_ratio = ONE;

    let mut count = 0;
    while count < balances.len() {
        // Swap fees are typically charged on 'token in', but there is no 'token in' here, so we apply it to
        // 'token out'. This results in slightly larger price impact.
        let mut amount_out_with_fee = 0;
        if (invariant_ratio_without_fees > balance_ratios_without_fee.get(count).unwrap())
        {
            let non_taxable_amount = mul_down(balances.get(count).unwrap(), complement(invariant_ratio_without_fees));
            let taxable_amount = amounts_out.get(count).unwrap() - non_taxable_amount;
            let taxable_amount_plus_fees = div_up(taxable_amount, complement(swap_fee_percentage));

            amount_out_with_fee = non_taxable_amount + taxable_amount_plus_fees;
        } else {
            amount_out_with_fee = amounts_out.get(count).unwrap();
        }

        let balance_ratio = div_down((balances.get(count).unwrap() - amount_out_with_fee), balances.get(count).unwrap());

        let invariant_ratio = mul_down(invariant_ratio, pow_down(balance_ratio, normalized_weights.get(count).unwrap()));

        count += 1;
    }
    invariant_ratio
}

pub fn calc_token_out_given_exact_bpt_in(
    balance: u64,
    normalized_weight: u64,
    bpt_amount_in: u64,
    bpt_total_supply: u64,
    swap_fee_percentage: u64,
) -> u64 {
    // exactBPTInForTokenOut                                                                //
    // a = amountOut                                                                        //
    // b = balance                     /      /    total_bpt - bptIn       \    (1 / w)  \   //
    // bptIn = bpt_amount_in    a = b * |  1 - | --------------------------  | ^           |  //
    // bpt = total_bpt                  \      \       total_bpt            /             /   //
    // w = weight                                                                           //
    // Token out, so we round down overall. The multiplication rounds down, but the power rounds up (so the base
    // rounds up). Because (total_bpt - bptIn) / total_bpt <= 1, the exponent rounds down.
    // Calculate the factor by which the invariant will decrease after burning BPTAmountIn
    // SCRIPT_TESTING
    // original --
    // let invariant_ratio = div_up((bpt_total_supply - bpt_amount_in), bpt_total_supply);
    // changed start --
    let mut invariant_ratio = div_up((bpt_total_supply + 10 - bpt_amount_in), bpt_total_supply + 10);
    invariant_ratio = 7;
    // changed end --
    require(invariant_ratio >= MIN_INVARIANT_RATIO, Error::MinBptInForTokenOut);

    // Calculate by how much the token balance has to decrease to match invariant_ratio
    let balance_ratio = pow_up(invariant_ratio, div_down(ONE, normalized_weight));

    // Because of rounding up, balance_ratio can be greater than one. Using complement prevents reverts.
    let amount_out_without_fee = mul_down(balance, complement(balance_ratio));

    // We can now compute how much excess balance is being withdrawn as a result of the virtual swaps, which result
    // in swap fees.
    // Swap fees are typically charged on 'token in', but there is no 'token in' here, so we apply it
    // to 'token out'. This results in slightly larger price impact. Fees are rounded up.
    let taxable_amount = mul_up(amount_out_without_fee, complement(normalized_weight));
    let non_taxable_amount = (amount_out_without_fee - taxable_amount);
    let taxable_amount_minus_fees = mul_up(taxable_amount, complement(swap_fee_percentage));
    (non_taxable_amount + taxable_amount_minus_fees)
}

pub fn calc_tokens_out_given_exact_bpt_in(
    balances: Vec<u64>,
    bpt_amount_in: u64,
    total_bpt: u64,
) -> Vec<u64> {
    // exactBPTInForTokensOut                                                                    //
    // (per token)                                                                               //
    // aO = amountOut                  /        bptIn         \                                  //
    // b = balance           a0 = b * | ---------------------  |                                 //
    // bptIn = bpt_amount_in             \       total_bpt       /                                  //
    // bpt = total_bpt                                                                            //
    // Since we're computing an amount out, we round down overall. This means rounding down on both the
    // multiplication and division.
    let bpt_ratio = div_down(bpt_amount_in, total_bpt);

    let mut amounts_out = ~Vec::new();

    let mut count = 0;
    while count < balances.len() {
        amounts_out.push(mul_down(balances.get(count).unwrap(), bpt_ratio));
        count += 1;
    }

    amounts_out
}

pub fn join_all_tokens_in_for_exact_bptout(
    balances: Vec<u64>,
    user_data: UserData,
) -> (u64, Vec<u64>) {
    // todo: abi.decode
    // let bpt_amount_out = user_data.allTokensInForExactBptOut();
    let bpt_amount_out = user_data.amount;
    // Note that there is no maximum amounts_in parameter: this is handled by `IVault.joinPool`.
    let amounts_in = calc_all_tokens_in_given_exact_bpt_out(balances, bpt_amount_out, TOTAL_SUPPLY);

    return (
        bpt_amount_out,
        amounts_in,
    );
}

/// MATH FUNCTIONS FOR FIXED POINTS

pub fn div_up(a: u64, b: u64) -> u64 {
    require(b != 0, Error::ZeroDivision);

    if (a == 0) {
        0
    } else {
        let a_inflated = a * ONE;
        require(a_inflated / a == ONE, Error::DivInternal); // mul overflow
        // The traditional div_up formula is:
        // div_up(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // div_up(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, which we already tested for.
        return ((a_inflated - 1) / b) + 1;
    }
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
        let max_error = mul_up(raw, MAX_POW_RELATIVE_ERROR) + ONE;

        return raw + max_error;
    }
}

// Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
//
// Useful when computing the complement for values with some level of relative error, as it strips this error and
// prevents intermediate negative values.
pub fn complement(x: u64) -> u64 {
    if x < ONE { ONE - x } else { 0 }
}


pub fn div_down(a: u64, b: u64) -> u64 { 
    require(b != 0, Error::ZeroDivision);
    if (a == 0) {
        0
    } else {
        let a_inflated = a * ONE;
        require(a_inflated / a == ONE, Error::DivInternal); // mul overflow
        a_inflated / b
    }
}

// Returns x^y, assuming both are fixed point numbers, rounding down. The result is guaranteed to not be above
// the true value (that is, the error pub pub fn expected - actual is always positive).
pub fn pow_down(x: u64, y: u64) -> u64 {
    // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
    // and 80/20 Weighted Pools
    if (y == ONE) {
        x
    } else if (y == TWO) {
        (x * x)
    } else if (y == FOUR) {
        let square = (x * x);
        (square * square)
    } else {
        let raw = x.pow(y);
        let max_error = mul_up(raw, MAX_POW_RELATIVE_ERROR) + ONE;

        if (raw < max_error) {
            0
        } else {
            return raw - max_error;
        }
    }
}

pub fn mul_up(a: u64, b: u64) -> u64 {
    let product = a * b;
    require(a == 0 || product / a == b, Error::MulOverflow);

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

pub fn mul_down(a: u64, b: u64) -> u64 {
    let product = a * b;
    require(a == 0 || product / a == b, Error::MulOverflow);

    product / ONE
}
