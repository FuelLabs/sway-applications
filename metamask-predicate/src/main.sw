predicate;

use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};

fn main(msg_hash: b256, signature: B512, expected_public_key: b256) -> bool {
    let res = ec_recover_address(signature, msg_hash);
    if res.is_ok() {
        if res.unwrap().value == expected_public_key {
            return true;
        }
    }
    false
}
