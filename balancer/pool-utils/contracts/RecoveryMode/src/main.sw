contract;


use std::{
    vec::Vec,
    revert::require,
    option::Option,
};
 
use FixedPoint::{mul_down, div_down};

abi RecoveryMode {
    #[storage(write)]fn enable_recovery_mode();
    #[storage(write)]fn disable_recovery_mode();
    #[storage(read)]fn in_recovery_mode() -> bool;
    #[storage(read)]fn _set_recovery_mode();
    #[storage(read)]fn _ensure_not_in_recovery_mode();
    // fn _do_recovery_mode_exit(
    //     balances: Vec<u64>,
    //     totalSupply: u64,
    //     userData: b256
    // ) -> (u64, Vec<u64>);
}

/**
 * @notice Handle storage and state changes for pools that support "Recovery Mode".
 *
 * @dev This is intended to provide a safe way to exit any pool during some kind of emergency, to avoid locking funds
 * in the event the pool enters a non-fnal state (i.e., some code that normally runs during exits is causing
 * them to revert).
 *
 * Recovery Mode is *not* the same as pausing the pool. The pause fn is only available during a short window
 * after factory deployment. Pausing can only be intentionally reversed during a buffer period, and the contract
 * will permanently unpause itself thereafter. Paused pools are completely disabled, in a kind of suspended animation,
 * until they are voluntarily or involuntarily unpaused.
 *
 * By contrast, a privileged account - typically a governance multisig - can place a pool in Recovery Mode at any
 * time, and it is always reversible. The pool is *not* disabled while in this mode: though of course whatever
 * condition prompted the transition to Recovery Mode has likely effectively disabled some fns. Rather,
 * a special "clean" exit is enabled, which runs the absolute minimum code necessary to exit proportionally.
 * In particular, stable pools do not attempt to compute the invariant (which is a complex, iterative calculation
 * that can fail in extreme circumstances), and no protocol fees are collected.
 *
 * It is critical to ensure that turning on Recovery Mode would do no harm, if activated maliciously or in error.
 */


// const _recovery_mode: bool = false;
storage{
    _recovery_mode: bool = false,
}

 /**
* @dev Sets the recoveryMode state, and emits the corresponding event. Can be overridden
* if a pool needs to detect when the Recovery Mode state changes.
*
* No complex code or external calls that could fail should be placed here, which could jeopardize
* the ability to enable and disable Recovery Mode.
*/
#[storage(write)]fn _set_recovery_mode(enabled: bool) {
    storage._recovery_mode = enabled;
}

// fn _compute_proportional_amounts_out(
//     balances: Vec<u64>,
//     totalSupply: u64,
//     bptAmountIn: u64
// ) -> Vec<u64> {
//     // exactBPTInForTokensOut                                                                    //
//     // (per token)                                                                               //
//     // aO = tokenAmountOut             /        bptIn         \                                  //
//     // b = tokenBalance      a0 = b * | ---------------------  |                                 //
//     // bptIn = bptAmountIn             \     bptTotalSupply    /                                 //
//     // bpt = bptTotalSupply                                                                      //

//     // Since we're computing an amount out, we round down overall. This means rounding down on both the
//     // multiplication and division.

//     let bptRatio = div_down(bptAmountIn, totalSupply);

//     let mut amountsOut = ~Vec::with_capacity(balances.len());
//     let mut count = 0;
//     while count < balances.len() {
//         amountsOut.swap(count, mul_down(balances.get(count).unwrap(), bptRatio))  
//     }
// }

impl RecoveryMode for Contract {
    /**
     * @notice Enable recovery mode, which enables a special safe exit path for LPs.
     * @dev Does not otherwise affect pool operations (beyond deferring payment of protocol fees), though some pools may
     * perform certain operations in a "safer" manner that is less likely to fail, in an attempt to keep the pool
     * running, even in a pathological state. Unlike the Pause operation, which is only available during a short window
     * after factory deployment, Recovery Mode can always be enableed.
     */
    #[storage(write)]
    fn enable_recovery_mode() {
        _set_recovery_mode(true);
    }

    /**
     * @notice Disable recovery mode, which disables the special safe exit path for LPs.
     * @dev Protocol fees are not paid while in Recovery Mode, so it should only remain active for as long as strictly
     * necessary.
     */
    #[storage(write)]
    fn disable_recovery_mode() {
        _set_recovery_mode(false);
    }

    /**
     * @notice Returns whether the pool is in Recovery Mode.
     */
    #[storage(read)]
    fn in_recovery_mode() -> bool {
        return storage._recovery_mode;
    }

    /**
     * @dev Reverts if the contract is not in Recovery Mode.
     */
    #[storage(read)]
    fn _set_recovery_mode(){
        require(storage._recovery_mode, "NOT_IN_recovery_mode");
    }

    /**
     * @dev Reverts if the contract is in Recovery Mode.
     */
    #[storage(read)]
    fn _ensure_not_in_recovery_mode() {
        require(!storage._recovery_mode, "IN_recovery_mode");
    }

    /**
     * @dev A minimal proportional exit, suitable as is for most pools: though not for pools with Phantom BPT
     * or other special considerations. Designed to be overridden if a pool needs to do extra processing,
     * such as scaling a stored invariant, or caching the new total supply.
     *
     * No complex code or external calls should be made in derived contracts that override this!
     */
    // fn _do_recovery_mode_exit(
    //     balances: Vec<u64>,
    //     totalSupply: u64,
    //     userData: b256
    // ) -> (u64, Vec<u64>) {
    //     // let bptAmountIn = recoveryModeExit(userData);
    //     // dummy value for bptAmountIn
    //     let mut bptAmountIn = ~u64::new();
    //     bptAmountIn = 74632874932874392;

    //     // let amountsOut = _compute_proportional_amounts_out(balances, totalSupply, bptAmountIn);

    //     return (bptAmountIn, amountsOut);
    // }
}
