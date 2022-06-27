library data_structures;

use std::{contract_id::ContractId, identity::Identity};

pub struct CallData {
    /// Specifies the amount of gas to forward to the arbitrary function call
    amount_of_gas_to_forward: u64,
    /// Asset Id of the coins to forward
    asset_id_of_coins_to_forward: ContractId,
    /// Stores information about an arbitrary contract function call
    memory_address: MemoryAddress,
    /// Number of coins to forward
    /// Coin type is specified by `asset_id_of_coins_to_forward`
    num_coins_to_forward: u64,
}

struct MemoryAddress {
    /// Contract id to call
    contract_id: ContractId,
    /// Data to pass called function
    function_data: u64,
    /// Function to call on the specified contract
    function_selector: u64,
}

pub struct Proposal {
    /// The needed percentage of yes votes to execute a proposal.
    /// 0 < acceptance_percentage <= 100
    acceptance_percentage: u64,
    /// Arbitrary call data for executing approved proposals
    call_data: CallData,
    /// Amount of blocks a proposal is valid for after creation
    end_height: u64,
    /// The number of no votes for a proposal
    no_votes: u64,
    /// The number of yes votes for a proposal
    yes_votes: u64,
}
