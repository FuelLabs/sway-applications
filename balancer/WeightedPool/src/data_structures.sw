library data_structures;

use std::{contract_id::ContractId, vec::Vec};

// In order to preserve backwards compatibility, make sure new join and exit kinds are added at the end of the enum.
pub enum JoinKind {
    Init: (),
    ExactToken: (),
    ExactTokensOut: (),
    InForExactTokensOut: (),
    Token: (),
}

pub enum ExitKind {
    ExactToken: (),
    ExactTokensOut: (),
    InForExactTokensOut: (),
    Token: (),
}

pub enum RequestKind {
    Init: (),
    ExactToken: (),
    ExactTokensOut: (),
    InForExactTokensOut: (),
    Token: (),
}

// Contains any pool-specific instructions needed to perform the calculations, such as
// the type of join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
pub struct UserData {
    kind: RequestKind,
    amount: u64,
    max_min_bpt_amount: u64,
    bpt_amount_in_out: u64,
    amounts_in_out: Vec<u64>,
}
