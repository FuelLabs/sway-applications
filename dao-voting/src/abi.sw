library abi;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::{Proposal, ProposalInfo};

abi DaoVoting {
    #[storage(read, write)] fn constructor(gov_token: ContractId);
    #[storage(read, write)]fn create_proposal(acceptance_percentage: u64, duration: u64, proposal_transaction: Proposal);
    #[storage(read, write)] fn deposit();
    #[storage(read, write)] fn withdraw(amount: u64);
    #[storage(read, write)]fn vote(approve: bool, proposal_id: u64, vote_amount: u64);
    #[storage(read, write)] fn execute(proposal_id: u64);
    #[storage(read, write)] fn unlock_votes(proposal_id: u64);
    #[storage(read)] fn balance() -> u64;
    #[storage(read)] fn user_balance(user: Identity) -> u64;
    #[storage(read)] fn user_votes(proposal_id: u64, user: Identity) -> u64;
    #[storage(read)] fn proposal(id: u64) -> ProposalInfo;
    #[storage(read)] fn governance_token_id() -> ContractId;
}
