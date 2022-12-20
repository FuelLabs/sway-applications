library data_structures;

/// Contains information on the NFT this contract holds.
pub struct NFTInfo {
    /// The identity which may unlock and withdraw the NFT.
    admin: Option<Identity>,
    /// The contract which manages the NFT.
    nft: ContractId,
    /// The id of the NFT that this contract holds.
    token_id: u64,
}

impl NFTInfo {
    pub fn new(admin: Option<Identity>, nft: ContractId, token_id: u64) -> Self {
        Self {
            nft,
            admin,
            token_id,
        }
    }
}
