library data_structures;

use std::{contract_id::ContractId, identity::Identity};

pub struct ProposalInfo {
    /// The needed percentage of yes votes to execute a proposal.
    /// 0 < acceptance_percentage <= 100
    acceptance_percentage: u64,
    /// Data necessary to execute an arbitrary transaction.
    proposal_transaction: Proposal,
    /// Amount of blocks a proposal is valid for after creation
    end_height: u64,
    /// The number of no votes for a proposal
    no_votes: u64,
    /// The number of yes votes for a proposal
    yes_votes: u64,
}

pub struct Proposal {
    /// Specifies the amount of gas to forward to the arbitrary function call
    gas: u64,
    /// Asset Id of the coins to forward
    asset: ContractId,
    /// Stores information about an arbitrary contract function call
    call_data: CallData,
    /// Number of coins to forward
    /// Coin type is specified by `asset_id_of_coins_to_forward`
    amount: u64,
}

struct CallData {
    /// Contract id to call
    id: ContractId,
    /// Data to pass called function
    arguments: u64,
    /// Function to call on the specified contract
    function_selector: u64,
}
