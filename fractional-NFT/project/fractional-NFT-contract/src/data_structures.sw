library data_structures;

pub struct NFTInfo {
    nft: ContractId,
    owner: Option<Identity>,
    token_id: u64,
}

impl NFTInfo {
    pub fn new(nft: ContractId, owner: Option<Identity>, token_id: u64) -> Self {
        NFTInfo {
            nft,
            owner,
            token_id
        }
    }
}
