library data_structures;

use std::contract_id::ContractId;
use core::ops::Eq;

/// Native asset the user must deposit and amount required for despoit
pub struct Asset {
    amount: u64,
    id: ContractId,
}

/// Metadata stored in mapping for identification and logic
pub struct User {
    approved: bool,

    /// The asset that the user has currently deposited in the contract
    asset: ContractId,
    // asset: Option<ContractId>, // enums not supported in storage

    /// Dummy value used to ensure that a caller is a valid user
    exists: bool,

    /// Value indicating whether the user currently holds a deposit in the contract
    deposited: bool,
}

impl Eq for User {
    fn eq(self, other: Self) -> bool {
        self.approved == other.approved && self.asset == other.asset && self.exists == other.exists && self.deposited == other.deposited
    }
}
