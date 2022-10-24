library utils;

use std::{constants::ZERO_B256, contract_id::ContractId, mem::addr_of};

// will remove once this function becomes a part of std-lib
pub fn bytecode_root(contract_id: ContractId) -> b256 {
    let root: b256 = ZERO_B256;

    asm(root_addr: addr_of(root), target: addr_of(contract_id.value)) {
        croo root_addr target;
        root_addr: b256
    }
}
