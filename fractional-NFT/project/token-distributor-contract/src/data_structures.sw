library data_structures;

pub enum DistributionState {
    Created: (),
    Closed: (),
    Distributing: (),
    Returning: (),
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
