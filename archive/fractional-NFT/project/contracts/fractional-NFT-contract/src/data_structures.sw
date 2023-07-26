library;

/// Contains information on the NFT this contract holds.
pub struct NFTInfo {
    /// The identity which may unlock and withdraw the NFT.
    admin: Option<Identity>,
    /// The contract which manages the NFT.
    asset_id: ContractId,
    /// The id of the NFT that this contract holds.
    token_id: u64,
}

impl NFTInfo {
    pub fn new(admin: Option<Identity>, asset_id: ContractId, token_id: u64) -> Self {
        Self {
            admin,
            asset_id,
            token_id,
        }
    }
}
