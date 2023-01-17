contract;

use chess::*;

abi Chess {
    fn test_function() -> bool;
}

impl Chess for Contract {
    fn test_function() -> bool {
        true
    }
}
