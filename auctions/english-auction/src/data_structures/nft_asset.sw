library nft_asset;

dep traits;

use ::errors::AssetError;
use traits::Asset;

pub struct NFTAsset {
    /// The `ContractId` of the NFT that the struct is representing.
    contract_id: ContractId,
    /// The token id of the NFT that the struct is representing.
    token_id: u64,
}

impl NFTAsset {
    fn new(contract_id: ContractId, token_id: u64) -> Self {
        NFTAsset {
            contract_id,
            token_id,
        }
    }

    fn token_id(self) -> u64 {
        self.token_id
    }
}

impl Asset for NFTAsset {
    fn amount(self) -> u64 {
        // TODO: This should return the number of tokens when StorageVec in structs is supported
        1
    }

    fn contract_id(self) -> ContractId {
        self.contract_id
    }
}

impl core::ops::Add for NFTAsset {
    fn add(self, other: Self) -> Self {
        // TODO: This should be implemented once StorageVec in structs is supported
        self
    }
}

impl core::ops::Eq for NFTAsset {
    fn eq(self, other: Self) -> bool {
        self.contract_id() == other.contract_id()
    }
}

impl core::ops::Ord for NFTAsset {
    fn gt(self, other: Self) -> bool {
        // TODO: This should be implemented once StorageVec in structs is supported
        false
    }
    fn lt(self, other: Self) -> bool {
        // TODO: This should be implemented once StorageVec in structs is supported
        false
    }
}
