library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::{CallData, Proposal};

abi DaoVoting {
    #[storage(read, write)] fn constructor(gov_token: ContractId) -> bool;
    #[storage(read, write)] fn deposit() -> bool;
    #[storage(read)] fn get_balance() -> u64;
    #[storage(read)] fn get_user_balance(user: Identity) -> u64;
    #[storage(read)] fn get_user_votes(user: Identity, proposal_id: u64) -> u64;
    #[storage(read, write)] fn add_proposal(voting_period: u64, approval_percentage: u64, proposal_data: CallData) -> bool;
    #[storage(read)] fn get_proposal(id: u64) -> Proposal;
    #[storage(read, write)] fn vote(proposal_id: u64, vote_amount: u64, is_yes_vote: bool) -> bool;
    #[storage(read, write)] fn execute(proposal_id: u64) -> bool;
    #[storage(read, write)] fn withdraw(amount: u64) -> bool;
    #[storage(read, write)] fn convert_votes_to_tokens(proposal_id: u64);
}