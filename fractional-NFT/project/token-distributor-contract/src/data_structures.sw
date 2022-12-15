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
            (

                DistributionState::Distributing,
                DistributionState::Distributing,
            ) => true,
            (DistributionState::Returning, DistributionState::Returning) => true,
            _ => false,
        }
    }
}

pub struct NFTInfo {
    nft: ContractId,
    owner: Option<Identity>,
    token_id: u64,
}

pub struct TokenDistribution {
    external_asset: ContractId,
    external_deposits: u64,
    nft: ContractId,
    owner: Option<Identity>,
    reserve_price: Option<u64>,
    state: DistributionState,
    token_id: u64,
    token_price: u64,
}

impl TokenDistribution {
    pub fn new(
        external_asset: ContractId,
        nft: ContractId,
        owner: Option<Identity>,
        reserve_price: Option<u64>,
        token_id: u64,
        token_price: u64,
    ) -> Self {
        TokenDistribution {
            external_asset,
            external_deposits: 0,
            nft,
            owner,
            reserve_price,
            state: DistributionState::Created,
            token_id,
            token_price,
        }
    }
}
