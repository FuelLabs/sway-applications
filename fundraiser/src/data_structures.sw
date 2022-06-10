library data_structures;

use std::{contract_id::ContractId, identity::Identity};
use core::ops::Eq;

pub enum State {
    Funding: (),
    Successful: (),
    Failed: (),
    Cancelled: (),
}

pub struct Campaign {
    author: Identity,
    asset: ContractId,
    claimed: bool,
    deadline: u64,
    state: State,
    target_amount: u64,
    total_pledge: u64,
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
