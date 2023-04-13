library;

pub enum DistributionState {
    Buyback: (),
    Distributed: (),
    Ended: (),
    Started: (),
}

impl core::ops::Eq for DistributionState {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (DistributionState::Buyback, DistributionState::Buyback) => true,
            (
                DistributionState::Distributed,
                DistributionState::Distributed,
            ) => true,
            (DistributionState::Ended, DistributionState::Ended) => true,
            (DistributionState::Started, DistributionState::Started) => true,
            _ => false,
        }
    }
}

pub struct TokenDistribution {
    /// The user which may withdraw payments, start a buyback, and withdraw the NFT.
    admin: Option<Identity>,
    /// The asset that is accepted as payment in exchange for fractionalized NFT tokens.
    external_asset: ContractId,
    /// The total amount of the `external_asset` that is available to withdraw by the admin.
    external_deposits: u64,
    /// The contract which manages the NFT held by the fractionalized NFT.
    nft_asset_id: ContractId,
    /// The price at which admin rights of the token distribution may be sold.
    reserve_price: Option<u64>,
    /// The current state of the distribution.
    state: DistributionState,
    /// The id of the NFT which is held by the fractionalized NFT.
    token_id: u64,
    /// The price for a single fractionalized NFT token.
    token_price: u64,
}

impl TokenDistribution {
    pub fn new(
        admin: Option<Identity>,
        external_asset: ContractId,
        nft_asset_id: ContractId,
        reserve_price: Option<u64>,
        token_id: u64,
        token_price: u64,
    ) -> Self {
        Self {
            admin,
            external_asset,
            external_deposits: 0,
            nft_asset_id,
            reserve_price,
            state: DistributionState::Started,
            token_id,
            token_price,
        }
    }
}
