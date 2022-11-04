contract;

dep interface;
dep utils;

use interface::FactoryWidePauseWindow;
use utils::{_INITIAL_PAUSE_WINDOW_DURATION, _BUFFER_PERIOD_DURATION};

abi FactoryWidePauseWindow {
    #[storage(read)]fn get_pause_configuration() -> (u64, u64);
}

const _INITIAL_PAUSE_WINDOW_DURATION: u64 = 90; // days;
const _BUFFER_PERIOD_DURATION: u64 = 30; // days;

// let _pools_pause_window_end_time: u64 = block.timestamp + _INITIAL_PAUSE_WINDOW_DURATION;
storage {
    _pools_pause_window_end_time: u64 = 123456789,
}

impl FactoryWidePauseWindow for Contract {
    #[storage(read)]fn get_pause_configuration() -> (u64, u64) {
        let mut pause_window_duration: u64 = 0;
        let mut buffer_period_duration: u64 = 0;
        // let current_time = block.timestamp;
        let current_time = 123456789;
        if (current_time < storage._pools_pause_window_end_time) {
            pause_window_duration = storage._pools_pause_window_end_time - current_time;
            buffer_period_duration = _BUFFER_PERIOD_DURATION;

            return(pause_window_duration, buffer_period_duration);
        } else {
            return(pause_window_duration, buffer_period_duration);
        }
    }
}
