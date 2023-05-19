predicate;

use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address};

configurable {
    /// Public key of signer
    PUBLIC_KEY: b256 = ZERO_B256,
}

fn main(msg_hash: b256, signature: B512) -> bool {
    let res = ec_recover_address(signature, msg_hash);
    if res.is_ok() && res.unwrap().value == PUBLIC_KEY {
        return true;
    }
    false
}
