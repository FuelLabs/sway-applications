contract;

use std::{
    revert::require,
    block::*,
};

use BalancerErrors::{PAUSE_WINDOW_EXPIRED, BUFFER_PERIOD_EXPIRED, PAUSED, NOT_PAUSED};

abi TemporarilyPausable {
    #[storage(read)]fn get_paused_state() -> (bool, u64, u64);
    #[storage(write)]fn _set_paused(paused: bool);
    fn _ensure_not_paused();
    fn _ensure_paused();
}


const _MAX_PAUSE_WINDOW_DURATION = 90;
const _MAX_BUFFER_PERIOD_DURATION = 30;
//comes from constructor so putting some dummy values
const _PAUSE_WINDOW_END_TIME = 1;
const _BUFFER_PERIOD_END_TIME = 2;


storage {
    paused: bool = false,
}


// These getters lead to reduced bytecode size by inlining the immutable variables in a single place.

fn _get_pause_window_end_time() -> u64 {
    return _PAUSE_WINDOW_END_TIME;
}

fn _get_buffer_period_end_time() -> u64 {
    return _BUFFER_PERIOD_END_TIME;
}


fn _is_not_paused() -> bool {
    // After the Buffer Period, the (inexpensive) timestamp check short-circuits the storage access.
    // return block_timestamp() > _get_buffer_period_end_time() || !storage.paused;
    true
}

impl TemporarilyPausable for Contract {
    /**
    * @dev Returns true if the contract is unpaused.
    *
    * Once the Buffer Period expires, the gas cost of calling this fn is reduced dramatically, as storage is no
    * longer accessed.
    */
    // The Pause Window and Buffer Period are timestamp-based: they should not be relied upon for sub-minute accuracy.
    // solhint-disable not-rely-on-time

    /**
     * @dev Returns the current contract pause status, as well as the end times of the Pause Window and Buffer
     * Period.
     */
    #[storage(read)]
    fn get_paused_state() -> (bool, u64, u64) {
        // storage.paused = !_is_not_paused();
        // let pauseWindowEndTime = _get_pause_window_end_time();
        // let bufferPeriodEndTime = _get_buffer_period_end_time();
        (!_is_not_paused(), _get_pause_window_end_time(), _get_buffer_period_end_time())


    }

    /**
     * @dev Sets the pause state to `paused`. The contract can only be paused until the end of the Pause Window, and
     * unpaused until the end of the Buffer Period.
     *
     * Once the Buffer Period expires, this fn reverts unconditionally.
     */
    #[storage(write)]
    fn _set_paused(paused: bool) {
        // if (paused) {
        //     require(block_timestamp() < _get_pause_window_end_time(), PAUSE_WINDOW_EXPIRED);
        // } else {
        //     require(block_timestamp() < _get_buffer_period_end_time(), BUFFER_PERIOD_EXPIRED);
        // }

        storage.paused = paused;
    }

    /**
     * @dev Reverts if the contract is paused.
     */
    fn _ensure_not_paused() {
        require(_is_not_paused(), PAUSED);
    }

    /**
     * @dev Reverts if the contract is not paused.
     */
    fn _ensure_paused() {
        require(!_is_not_paused(), NOT_PAUSED);
    }
}
