library data_structures;

use core::ops::Eq;

/// Used to track the total amount pledged to an asset
pub struct AssetInfo {
    /// The amount that is currently pledged
    amount: u64,
    /// Given that an asset can be set in a campaign and never pledged to,
    /// we need this field to handle control flow
    exists: bool,
}

/// Used to track the campaigns that a user has created
pub struct Campaign {
    /// The unique identifier for the campaign
    id: u64,
}

impl Campaign {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

/// General data structure containing information about a campaign
pub struct CampaignInfo {
    /// The user who has created the campaign
    author: Identity,
    /// The asset that this campaign accepts as a deposit
    asset: ContractId,
    /// The user to whom the funds will be sent to upon a successful campaign
    beneficiary: Identity,
    // TODO: document
    state: State,
    /// The end time for the campaign after which it becomes locked
    deadline: u64,
    /// The amount needed to deem the campaign a success
    target_amount: u64,
    /// The current amount pledged used to measure against the target_amount
    total_pledge: u64,
}

impl CampaignInfo {
    pub fn new(
        asset: ContractId,
        author: Identity,
        beneficiary: Identity,
        deadline: u64,
        target_amount: u64,
    ) -> Self {
        Self {
            asset,
            author,
            beneficiary,
            state: State::Funding,
            deadline,
            target_amount,
            total_pledge: 0,
        }
    }
}

/// Used to track the amount pledged by a user to a specific campaign
pub struct Pledge {
    /// The amount pledged to a campaign
    amount: u64,
    /// The unique identifier for the campaign
    id: u64,
}

impl Pledge {
    pub fn new(amount: u64, id: u64) -> Self {
        Self { amount, id }
    }
}

pub enum State {
    Funding: (),
    Cancelled: (),
    Claimed: (),
}

impl Eq for State {
    fn eq(self, other: State) -> bool {
        match (self, other) {
            (State::Funding, State::Funding) => true,
            (State::Cancelled, State::Cancelled) => true,
            (State::Claimed, State::Claimed) => true,
            _ => false,
        }
    }
}
