library data_structures;

pub enum DistributionState {
    Created: (),
    Closed: (),
    Distributing: (),
    Returning: (),
}

impl core::ops::Eq for DistributionState {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (DistributionState::Created, DistributionState::Created) => true,
            (DistributionState::Closed, DistributionState::Closed) => true,
            (DistributionState::Distributing, DistributionState::Distributing) => true,
            (DistributionState::Returning, DistributionState::Returning) => true,
            _ => false,
        }
    }
}

pub struct TokenDistribution {
    buy_asset: ContractId,
    nft: ContractId,
    reserve_price: u64,
    state: DistributionState,
    token_id: u64,
    token_price: u64,
}

impl TokenDistribution {
    pub fn new(
        buy_asset: ContractId,
        nft: ContractId,
        reserve_price: u64,
        token_id: u64,
        token_price: u64,
    ) -> Self {
        TokenDistribution {
            buy_asset,
            nft,
            reserve_price,
            state: DistributionState::Created,
            token_id,
            token_price,
        }
    }
}
