library data_structures;

use std::{chain::auth::Sender, contract_id::ContractId};
use core::ops::Eq;

pub enum Initialized {
    True: (),
    False: (),
}

pub enum State {
    Funding: (),
    Successful: (),
    Failed: (),
    Cancelled: (),
}

pub struct Campaign {
    author: Sender,
    asset: ContractId,
    claimed: bool,
    deadline: u64,
    state: State,
    target_amount: u64,
    total_pledge: u64,
}

// impl Eq for Initialized {
//     fn eq(self, other: Self) -> bool {
//         match (self, other) {
//             (Initialized::True, Initialized::True) => true,
//             (Initialized::False, Initialized::False) => true,
//             _ => false,
//         }
//     }
// }

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
