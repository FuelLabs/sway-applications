library;

use core::ops::Eq;

/// Trusted 3rd party who handles the resolution of a dispute.
pub struct Arbiter {
    /// Address identifying the arbiter.
    address: Identity,
    /// The asset that the arbiter will be paid in upon resolution.
    asset: ContractId,
    /// The quantity of asset to be taken as payment.
    fee_amount: u64,
}

impl Eq for Arbiter {
    fn eq(self, other: Self) -> bool {
        self.address == other.address && self.asset == other.asset && self.fee_amount == other.fee_amount
    }
}

/// Represents the amount and type of assets being escrowed.
pub struct Asset {
    /// Amount of asset the user must deposit.
    amount: u64,
    /// The id used to identify the asset for deposit.
    id: ContractId,
}

/// Represents the user who is buying the asset.
pub struct Buyer {
    /// Address identifying the buyer.
    address: Identity,
    /// The asset that the user has currently deposited in the contract.
    asset: Option<ContractId>,
    // Minor data duplication allows us to forego validating unique assets upon escrow creation
    // otherwise the same asset with different values can be added which, if handled incorrectly,
    // may allow the user to drain the contract
    /// The amount of asset that has been deposited.
    deposited_amount: u64,
}

/// All the information about an escrow transaction.
pub struct EscrowInfo {
    /// Trusted 3rd party who handles the resolution of a dispute.
    arbiter: Arbiter,
    /// Total number of assets the escrow accepts.
    asset_count: u64,
    /// The authorized user who is able to make a payment into the escrow.
    buyer: Buyer,
    /// End height after which the buyer can no longer deposit and the seller can take payment.
    deadline: u64,
    /// Marker set by the buyer to lock the escrow and prevent the seller from taking payment.
    disputed: bool,
    /// Index of the first asset in storage vec `assets`.
    first_asset_index: u64,
    /// The authorized user who is the recipient of payments made by the buyer.
    seller: Seller,
    /// Mechanism used to manage the control flow of the escrow.
    state: State,
}

impl EscrowInfo {
    /// Creates a new EscrowInfo struct.
    ///
    /// # Arguments
    ///
    /// * `arbiter`: [Arbiter] - The trusted 3rd party who handles the resolution of a dispute.
    /// * `asset_count`: [u64] - Total number of assets the escrow accepts.
    /// * `buyer`: [Buyer] - The authorized user who is able to make a payment into the escrow.
    /// * `deadline`: [u64] - End height after which the buyer can no longer deposit and the seller can take payment.
    /// * `first_asset_index`: [u64] - Index of the first asset in storage vec `assets`.
    /// * `seller`: [Seller] - The authorized user who is the recipient of payments made by the buyer.
    ///
    /// # Returns
    ///
    /// * [EscrowInfo] - The newly created EscrowInfo struct.
    pub fn new(
        arbiter: Arbiter,
        asset_count: u64,
        buyer: Identity,
        deadline: u64,
        first_asset_index: u64,
        seller: Identity,
    ) -> Self {
        Self {
            arbiter,
            asset_count,
            buyer: Buyer {
                address: buyer,
                asset: Option::None,
                deposited_amount: 0,
            },
            deadline,
            disputed: false,
            first_asset_index,
            seller: Seller {
                address: seller,
            },
            state: State::Pending,
        }
    }
}

/// Represents the user who is selling the asset.
pub struct Seller {
    /// Address identifying the seller.
    address: Identity,
}

/// Represents the state of an escrow transaction.
pub enum State {
    /// Escrow has been created however the deposit has not been sent to either buyer or seller.
    Pending: (),
    /// The deposit has been sent to either the buyer or seller.
    Completed: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Pending, State::Pending) => {
                true
            },
            (State::Completed, State::Completed) => {
                true
            },
            _ => {
                false
            },
        }
    }
}
