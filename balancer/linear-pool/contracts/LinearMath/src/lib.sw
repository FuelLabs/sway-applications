library LinearMath;

use std:: {
    vec::Vec,
    option::Option
};

use FixedPoint::{
    add,
    sub,
    ONE,
    mul_down,
    mul_up
};

use math::{
    div_down,
    div_up,
    mul
};



// A Linear Pool holds three tokens: the main token, the wrapped token, and the Pool share token (BPT). It is
// possible to exchange any of these tokens for any of the other two (so we have three trading pairs) in both
// directions (the first token of each pair can be bought or sold for the second) and by specifying either the input
// or output amount (typically referred to as 'given in' or 'given out'). A full description thus requires
// 3*2*2 = 12 fns.
// Wrapped tokens have a known, trusted exchange rate to main tokens. All fns here assume such a rate has
// already been applied, meaning main and wrapped balances can be compared as they are both expressed in the same
// units (those of main token).
// Additionally, Linear Pools feature a lower and upper target that represent the desired range of values for the
// main token balance. Any action that moves the main balance away from this range is charged a proportional fee,
// and any action that moves it towards this range is incentivized by paying the actor using these collected fees.
// The collected fees are not stored in a separate data structure: they are a pub fn of the current main balance,
// targets and fee percentage. The main balance sans fees is known as the 'nominal balance', which is always smaller
// than the real balance except when the real balance is within the targets.
// The rule under which Linear Pools conduct trades between main and wrapped tokens is by keeping the sum of nominal
// main balance and wrapped balance constant: this value is known as the 'invariant'. BPT is backed by nominal
// reserves, meaning its supply is proportional to the invariant. As the wrapped token appreciates in value and its
// exchange rate to the main token increases, so does the invariant and thus the value of BPT (in main token units).

pub struct Params {
    fee: u64,
    lowerTarget: u64,
    upperTarget: u64,
}

pub fn _calc_bpt_out_per_main_in(
    mainIn: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount out, so we round down overall.

    if (bptSupply == 0) {
        // BPT typically grows in the same ratio the invariant does. The first time liquidity is added however, the
        // BPT supply is initialized to equal the invariant (which in this case is just the nominal main balance as
        // there is no wrapped balance).
        return _to_nominal(mainIn, params);
    }

    let previousNominalMain: u64 = _to_nominal(mainBalance, params);
    let afterNominalMain: u64 = _to_nominal(add(mainBalance, mainIn), params);
    let deltaNominalMain: u64 = sub(afterNominalMain, previousNominalMain);
    let invariant: u64 = _calc_invariant(previousNominalMain, wrappedBalance);
    return div_down(mul(bptSupply, deltaNominalMain), invariant);
}

pub fn _calc_bpt_in_per_main_out(
    mainOut: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let afterNominalMain = _to_nominal(sub(mainBalance, mainOut), params);
    let deltaNominalMain = sub(previousNominalMain, afterNominalMain);
    let invariant = _calc_invariant(previousNominalMain, wrappedBalance);
    return div_up(mul(bptSupply, deltaNominalMain), invariant);
}

pub fn _calc_wrapped_out_per_main_in(
    mainIn: u64,
    mainBalance: u64,
    params: Params
)->u64 {
    // Amount out, so we round down overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let afterNominalMain = _to_nominal(add(mainBalance ,mainIn), params);
    return sub(afterNominalMain, previousNominalMain);
}

pub fn _calc_wrapped_in_per_main_out(
    mainOut: u64,
    mainBalance: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let afterNominalMain = _to_nominal(sub(mainBalance,mainOut), params);
    return sub(previousNominalMain,afterNominalMain);
}

pub fn _calc_main_in_per_bpt_out(
    bptOut: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    if (bptSupply == 0) {
        // BPT typically grows in the same ratio the invariant does. The first time liquidity is added however, the
        // BPT supply is initialized to equal the invariant (which in this case is just the nominal main balance as
        // there is no wrapped balance).
        return _from_nominal(bptOut, params);
    }

    let previousNominalMain = _to_nominal(mainBalance, params);
    let invariant = _calc_invariant(previousNominalMain, wrappedBalance);
    let deltaNominalMain = div_up(mul(invariant, bptOut), bptSupply);
    let afterNominalMain = add(previousNominalMain,deltaNominalMain);
    let newMainBalance = _from_nominal(afterNominalMain, params);
    return sub(newMainBalance,mainBalance);
}

pub fn _calc_main_out_per_bpt_in(
    bptIn: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount out, so we round down overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let invariant = _calc_invariant(previousNominalMain, wrappedBalance);
    let deltaNominalMain = div_down(mul(invariant, bptIn), bptSupply);
    let afterNominalMain = sub(previousNominalMain,deltaNominalMain);
    let newMainBalance = _from_nominal(afterNominalMain, params);
    return sub(mainBalance,newMainBalance);
}

pub fn _calc_main_out_per_wrapped_in(
    wrappedIn: u64,
    mainBalance: u64,
    params: Params
) ->u64 {
    // Amount out, so we round down overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let afterNominalMain = sub(previousNominalMain,wrappedIn);
    let newMainBalance = _from_nominal(afterNominalMain, params);
    return sub(mainBalance,newMainBalance);
}

pub fn _calc_main_in_per_wrapped_out(
    wrappedOut: u64,
    mainBalance: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    let previousNominalMain = _to_nominal(mainBalance, params);
    let afterNominalMain = add(previousNominalMain, wrappedOut);
    let newMainBalance = _from_nominal(afterNominalMain, params);
    return sub(newMainBalance, mainBalance);
}

pub fn _calc_bpt_out_per_wrapped_in(
    wrappedIn: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount out, so we round down overall.

    if (bptSupply == 0) {
        // BPT typically grows in the same ratio the invariant does. The first time liquidity is added however, the
        // BPT supply is initialized to equal the invariant (which in this case is just the wrapped balance as
        // there is no main balance).
        return wrappedIn;
    }

    let nominalMain = _to_nominal(mainBalance, params);
    let previousInvariant = _calc_invariant(nominalMain, wrappedBalance);

    let newWrappedBalance = add(wrappedBalance, wrappedIn);
    let newInvariant = _calc_invariant(nominalMain, newWrappedBalance);

    let newBptBalance = div_down(mul(bptSupply, newInvariant), previousInvariant);

    return sub(newBptBalance, bptSupply);
}

pub fn _calc_bpt_in_per_wrapped_out(
    wrappedOut: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    let nominalMain = _to_nominal(mainBalance, params);
    let previousInvariant = _calc_invariant(nominalMain, wrappedBalance);

    let newWrappedBalance = sub(wrappedBalance, wrappedOut);
    let newInvariant = _calc_invariant(nominalMain, newWrappedBalance);

    let newBptBalance = div_down(mul(bptSupply, newInvariant), previousInvariant);

    return sub(bptSupply, newBptBalance);
}

pub fn _calc_wrapped_in_per_bpt_out(
    bptOut: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount in, so we round up overall.

    if (bptSupply == 0) {
        // BPT typically grows in the same ratio the invariant does. The first time liquidity is added however, the
        // BPT supply is initialized to equal the invariant (which in this case is just the wrapped balance as
        // there is no main balance).
        return bptOut;
    }

    let nominalMain = _to_nominal(mainBalance, params);
    let previousInvariant = _calc_invariant(nominalMain, wrappedBalance);

    let newBptBalance = add(bptSupply, bptOut);
    let newWrappedBalance = sub(div_up(mul(newBptBalance, previousInvariant), bptSupply), nominalMain);

    return sub(newWrappedBalance, wrappedBalance);
}

pub fn _calc_wrapped_out_per_bpt_in(
    bptIn: u64,
    mainBalance: u64,
    wrappedBalance: u64,
    bptSupply: u64,
    params: Params
) ->u64 {
    // Amount out, so we round down overall.

    let nominalMain = _to_nominal(mainBalance, params);
    let previousInvariant = _calc_invariant(nominalMain, wrappedBalance);

    let newBptBalance = sub(bptSupply, bptIn);
    let newWrappedBalance = sub(div_up(mul(newBptBalance, previousInvariant), bptSupply), nominalMain);

    return sub(wrappedBalance, newWrappedBalance);
}

pub fn _calc_invariant(nominalMainBalance: u64 , wrappedBalance: u64) ->u64 {
    return add(nominalMainBalance, wrappedBalance);
}

pub fn _to_nominal(real: u64, params: Params) ->u64 {
    // Fees are always rounded down: either direction would work but we need to be consistent, and rounding down
    // uses less gas.

    if (real < params.lowerTarget) {
        let fees = mul_down((params.lowerTarget - real), params.fee);
        return sub(real, fees);
    } else if (real <= params.upperTarget) {
        return real;
    } else {
        let fees = mul_down((real - params.upperTarget), params.fee);
        return sub(real, fees);
    }
}

pub fn _from_nominal(nominal: u64, params: Params) ->u64 {
    // Since real = nominal + fees, rounding down fees is equivalent to rounding down real.

    if (nominal < params.lowerTarget) {
        return div_down((add(nominal, mul_down(params.fee, params.lowerTarget))), add(ONE, params.fee));
    } else if (nominal <= params.upperTarget) {
        return nominal;
    } else {
        return div_down(sub(nominal, mul_down(params.fee, params.upperTarget)), sub(ONE, params.fee));
    }
}

pub fn _calc_tokens_out_given_exact_bpt_in(
    balances: Vec<u64>,
    bptAmountIn: u64,
    bptTotalSupply: u64,
    bptIndex: u64
)->Vec<u64> {
    // /**********************************************************************************************
    // exactBPTInForTokensOut                                                                    //
    // (per token)                                                                               //
    // aO = tokenAmountOut             /        bptIn         \                                  //
    // b = tokenBalance      a0 = b * | ---------------------  |                                 //
    // bptIn = bptAmountIn             \     bptTotalSupply    /                                 //
    // bpt = bptTotalSupply                                                                      //
    // **********************************************************************************************/
    // Since we're computing an amount out, we round down overall. This means rounding down on both the
    // multiplication and division.

    let bptRatio = div_down(bptAmountIn, bptTotalSupply);

    let mut amountsOut: Vec<u64> = ~Vec::with_capacity(balances.len());
    let mut counter = 0;
    while counter < balances.len() {
        if counter != bptIndex {
            // BPT is skipped as those tokens are not the LPs, but rather the preminted and undistributed amount.
            amountsOut.insert(counter, mul_down(balances.get(counter).unwrap(), bptRatio))
        }
        counter = counter + 1;
    }
    return amountsOut;
}

