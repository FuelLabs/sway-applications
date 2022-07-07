library utils;

dep data_structures;
dep events;

use data_structures::{Arbiter, ArbiterProposal, EscrowInfo};
use events::{ChangedArbiterEvent, ProposedArbiterEvent};
use std::{identity::Identity, logging::log, option::Option};

pub fn change_arbiter(arbiter: Arbiter, escrow: EscrowInfo, identifier: u64, proposal: ArbiterProposal, user: Identity, user_proposal: Option<Arbiter>) -> (bool, EscrowInfo, ArbiterProposal) {
    let mut escrow = escrow;
    let mut proposal = proposal;
    let mut update_state = false;

    if user_proposal.is_none() || arbiter.address != user_proposal.unwrap().address || arbiter.fee_percentage != user_proposal.unwrap().fee_percentage {
        if user == escrow.buyer.address {
            proposal.buyer = Option::Some( arbiter );
        } else {
            proposal.seller = Option::Some( arbiter );
        }
        log(ProposedArbiterEvent { arbiter, identifier, user });
    } else {
        escrow.arbiter = arbiter.address;
        escrow.arbiter_fee_percentage = arbiter.fee_percentage;

        proposal.buyer = Option::None;
        proposal.seller = Option::None;

        update_state = true;
        log(ChangedArbiterEvent { address: arbiter.address, fee_percentage: arbiter.fee_percentage, identifier });
    }

    (update_state, escrow, proposal)
}
