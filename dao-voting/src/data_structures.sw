library data_structures;

use std::{contract_id::ContractId, identity::Identity};

pub struct CallData {
    memory_address: MemoryAddress,
    asset_id_of_coins_to_forward: ContractId,
    num_coins_to_forward: u64,
    amount_of_gas_to_forward: u64,
}

struct MemoryAddress {
    contract_id: ContractId,
    function_selector: u64,
    function_data: u64,
}

pub struct Proposal {
    yes_votes: u64,
    no_votes: u64,
    approval_percentage: u64,
    end_height: u64,
    call_data: CallData,
}
