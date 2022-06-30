library data_structures;

dep errors;

use errors::AssetError;
use std::{
    assert::require,
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    storage::StorageMap,
};

pub enum Asset {
    NFTAsset: NFTAsset,
    TokenAsset: TokenAsset,
}

pub enum State {
    Closed: (),
    Open: (),
}

pub struct Auction {
    /// The asset which will be accepted in return for `sell_asset`.
    /// On initalization, the amount will be set to 0 and the `contract_id` will be set to the
    /// `ContractId` of the asset in return.
    buy_asset: Asset,
    /// The current highest bidder of the auction. When the auction is over, this is the winner.
    /// If no one bid on the auction or the auction is canceled, this will be `None`.
    highest_bidder: Option<Identity>,
    /// The block at which the auction should end
    end_block: u64,
    /// The starting price for the auction to start. This can be 0.
    inital_price: u64,
    /// The reserve price for the auction. When this amount is met, the auction will automatically
    /// close and the `sell_asset` will be sold.
    reserve_price: Option<u64>,
    /// The asset that is being auctioned off. This can be a native token or an NFT.
    sell_asset: Asset,
    /// The `Identity` of the seller or owner of the auction. Only the seller can cancel an auction.
    seller: Identity,
    /// The state of the auction describing if it is open or closed.
    state: State,
}

pub struct NFTAsset {
    /// The `ContractId` of the NFT that the struct is representing.
    contract_id: ContractId,
    // TODO: This needs to be a Vec to support mutliple NFTs.
    /// The token id of the NFT that the struct is representing.
    token_ids: u64,
}

pub struct TokenAsset {
    /// The amount of the native asset that the struct is representing.
    amount: u64,
    /// The `ContractId` of the native asset that the struct is representing.
    contract_id: ContractId,
}

pub trait ReturnsAmount {
    pub fn amount(self) -> u64;
}

pub trait ReturnsContractId {
    pub fn contract_id(self) -> ContractId;
}

impl core::ops::Add for Asset {
    pub fn add(self, other: Self) -> Self {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                require(nft_asset1.contract_id == nft_asset2.contract_id, AssetError::AssetsAreNotTheSame);
                // TODO: Combine vecs
                self
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                require(token_asset1.contract_id == token_asset2.contract_id, AssetError::AssetsAreNotTheSame);
                let total_amount = token_asset1.amount + token_asset2.amount;
                let token = TokenAsset {
                    amount: total_amount, contract_id: token_asset1.contract_id
                };
                Asset::TokenAsset(token)
            },
            _ => {
                self
            },
        }
    }
}

impl ReturnsAmount for Asset {
    pub fn amount(self) -> u64 {
        match self {
            Asset::NFTAsset(nft_asset) => {
                // TODO: Return Vec length
                1
            },
            Asset::TokenAsset(token_asset) => {
                token_asset.amount
            },
        }
    }
}

impl ReturnsContractId for Asset {
    pub fn contract_id(self) -> ContractId {
        match self {
            Asset::NFTAsset(nft_asset) => {
                nft_asset.contract_id
            },
            Asset::TokenAsset(token_asset) => {
                token_asset.contract_id
            },
        }
    }
}

impl core::ops::Eq for Asset {
    pub fn eq(self, other: Self) -> bool {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                nft_asset1.contract_id == nft_asset2.contract_id
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                token_asset1.contract_id == token_asset2.contract_id
            },
            _ => {
                false
            },
        }
    }
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Open, State::Open) => true, (State::Closed, State::Closed) => true, _ => false, 
        }
    }
}

impl core::ops::Ord for Asset {
    pub fn gt(self, other: Self) -> bool {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                // TODO: Compare NFT token ID Vec length
                nft_asset1.contract_id == nft_asset2.contract_id
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                token_asset1.contract_id == token_asset2.contract_id && token_asset1.amount > token_asset2.amount
            },
            _ => {
                false
            },
        }
    }

    pub fn lt(self, other: Self) -> bool {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                // TODO: Compare NFT token ID Vec length
                nft_asset1.contract_id == nft_asset2.contract_id
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                token_asset1.contract_id == token_asset2.contract_id && token_asset1.amount < token_asset2.amount
            },
            _ => {
                false
            },
        }
    }
}

impl core::ops::Subtract for Asset {
    pub fn subtract(self, other: Self) -> Self {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                require(nft_asset1.contract_id == nft_asset2.contract_id, AssetError::AssetsAreNotTheSame);
                // TODO: Remove differences in the Vecs
                self
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                require(token_asset1.contract_id == token_asset2.contract_id, AssetError::AssetsAreNotTheSame);
                let total_amount = token_asset1.amount - token_asset2.amount;
                let token = TokenAsset {
                    amount: total_amount, contract_id: token_asset1.contract_id
                };
                Asset::TokenAsset(token)
            },
            _ => {
                self
            },
        }
    }
}
