library utils;

use std::{constants::ZERO_B256, mem::addr_of};

// this is now part of std-lib in `forc 0.28`. will remove once forc version is bumped
// https://github.com/FuelLabs/sway/pull/3082/
pub fn bytecode_root(contract_id: ContractId) -> b256 {
    let root: b256 = ZERO_B256;

    asm(root_addr: addr_of(root), target: addr_of(contract_id.value)) {
        croo root_addr target;
        root_addr: b256
    }
}
