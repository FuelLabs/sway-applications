library utils;

dep data_structures;
dep events;

use data_structures::{Arbiter, ArbiterProposal, EscrowInfo};
use events::{ChangedArbiterEvent, ProposedArbiterEvent};
use std::{contract_id::ContractId, identity::Identity, logging::log, option::Option};

/// Helper used to update the escrow and proposal with a new arbiter 
///
/// # Arguments
///
/// * `arbiter` - A third party which decides how a dispute is resolved
/// * `escrow` - Data describing an existing escrow
/// * `identifier` - Identifier used to find a specific escrow
/// * `proposal` - Data containing proposals by the buyer and seller
/// * `user` - The buyer or seller
/// * `user_proposal` - Specific proposal from either the buyer or seller
///
/// # Returns
///
/// * `escrow` - Data describing an existing escrow
/// * `proposal` - Data containing proposals by the buyer and seller
pub fn change_arbiter(arbiter: Arbiter, escrow: EscrowInfo, identifier: u64, proposal: ArbiterProposal, user: Identity, user_proposal: Option<Arbiter>) -> (EscrowInfo, ArbiterProposal) {
    let mut escrow = escrow;
    let mut proposal = proposal;

    if user_proposal.is_none() || arbiter.address != user_proposal.unwrap().address || arbiter.asset != user_proposal.unwrap().asset || arbiter.fee_amount != user_proposal.unwrap().fee_amount {
        if user == escrow.buyer.address {
            proposal.buyer = Option::Some( arbiter );
        } else {
            proposal.seller = Option::Some( arbiter );
        }
        log(ProposedArbiterEvent { arbiter, identifier, user });
    } else {
        // TODO: transfer funds back to seller?
        escrow.arbiter = arbiter;

        proposal.buyer = Option::None;
        proposal.seller = Option::None;

        log(ChangedArbiterEvent { arbiter, identifier, user });
    }

    (escrow, proposal)
}
