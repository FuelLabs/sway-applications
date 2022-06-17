library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::Proposal;

abi DaoVoting {
    fn constructor(gov_token: ContractId) -> bool;
    fn deposit() -> bool;
    fn get_balance() -> u64;
    fn get_user_balance(user: Identity) -> u64;
    fn get_user_votes(user: Identity) -> u64;
    fn add_proposal(voting_period: u64, approval_percentage: u64, proposal_data: b256) -> bool;
    fn get_proposal(id: u64) -> Proposal;
    fn vote(proposal_id: u64, vote_amount: u64, is_yes_vote: bool) -> bool;
    fn execute(proposal_id: u64) -> bool;
    fn withdraw(amount: u64) -> bool;
}