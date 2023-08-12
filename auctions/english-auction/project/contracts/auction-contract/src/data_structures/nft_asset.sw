library;

use ::errors::AssetError;
use ::data_structures::traits::Asset;

/// Represents an NFT asset.
pub struct NFTAsset {
    /// The `ContractId` of the NFT that the struct is representing.
    asset_id: ContractId,
    /// The token id of the NFT that the struct is representing.
    token_id: u64,
}

impl NFTAsset {
    /// Creates a new `NFTAsset` struct.
    ///
    /// # Arguments
    ///
    /// * `asset_id`: [ContractId] - The AssetId of the NFT that the struct is representing.
    /// * `token_id`: [u64] - The token id of the NFT that the struct is representing.
    ///
    /// # Returns
    ///
    /// * [NFTAsset] - The newly created `NFTAsset` struct.
    fn new(asset_id: ContractId, token_id: u64) -> Self {
        NFTAsset {
            asset_id,
            token_id,
        }
    }

    /// Returns the token_id of the NFT that the struct is representing.
    ///
    /// # Returns
    ///
    /// * [u64] - The token id of the NFT that the struct is representing.
    pub fn token_id(self) -> u64 {
        self.token_id
    }
}

impl Asset for NFTAsset {
    fn amount(self) -> u64 {
        // TODO: This should return the number of tokens when StorageVec in structs is supported
        1
    }

    fn asset_id(self) -> ContractId {
        self.asset_id
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
        self.asset_id() == other.asset_id()
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
