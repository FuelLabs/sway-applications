library data_structures;

use std::{contract_id::ContractId, identity::Identity};
use core::ops::Eq;

pub struct Campaign {
    author: Identity,
    asset: ContractId,
    beneficiary: Identity,
    claimed: bool,
    deadline: u64,
    // state: State,
    state: u64, // workaround until Eq on self works
    target_amount: u64,
    total_pledge: u64,
}

pub enum State {
    Funding: (),
    Successful: (),
    Failed: (),
    Cancelled: (),
}

pub struct UserCampaigns {
    active: [u64;
    1],
    completed: [u64;
    1],
}

// impl Eq for State {
//     fn eq(self, other: Self) -> bool {
//         match (self, other) {
//             (State::Funding, State::Funding) => true,
//             (State::Successful, State::Successful) => true,
//             (State::Failed, State::Failed) => true,
//             (State::Cancelled, State::Cancelled) => true,
//             _ => false,
//         }
//     }
// }
