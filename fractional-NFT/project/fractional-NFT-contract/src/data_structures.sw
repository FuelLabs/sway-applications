library data_structures;

/// Contains information on the NFT this contract holds.
pub struct NFTInfo {
    /// The contract which manages the NFT.
    nft: ContractId,
    /// The identity which may unlock and withdraw the NFT.
    owner: Option<Identity>,
    /// The id of the NFT that this contract holds.
    token_id: u64,
}

impl NFTInfo {
    pub fn new(nft: ContractId, owner: Option<Identity>, token_id: u64) -> Self {
        NFTInfo {
            nft,
            owner,
            token_id,
        }
    }
}
