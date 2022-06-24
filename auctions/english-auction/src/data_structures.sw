library data_structures;

use std:: {
    assert::assert, contract_id::ContractId, identity::Identity, option::Option, storage::StorageMap
};

pub struct Auction {
    buy_asset: Asset,
    bidder: Option<Identity>,
    end_block: u64,
    inital_price: u64,
    reserve_price: Option<u64>,
    sell_asset: Asset,
    seller: Identity,
    state: u64,
}

pub enum Asset {
    NFTAsset: NFTAsset,
    TokenAsset: TokenAsset,
}

pub struct NFTAsset {
    contract_id: ContractId,
    token_ids: u64,
}

pub struct TokenAsset {
    amount: u64,
    contract_id: ContractId,
}

pub trait AssetTraits {
    pub fn amount(self) -> u64;
    pub fn contract_id(self) -> ContractId;
}

impl AssetTraits for Asset {
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

impl core::ops::Ord for Asset {
    pub fn gt(self, other: Self) -> bool {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                nft_asset1.contract_id == nft_asset2.contract_id && nft_asset1.token_ids > nft_asset2.token_ids
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
                nft_asset1.contract_id == nft_asset2.contract_id && nft_asset1.token_ids < nft_asset2.token_ids
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

impl core::ops::Add for Asset {
    pub fn add(self, other: Self) -> Self {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                assert(nft_asset1.contract_id == nft_asset2.contract_id);
                // TODO: Combine vecs
                self
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                assert(token_asset1.contract_id == token_asset2.contract_id);
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

impl core::ops::Subtract for Asset {
    pub fn subtract(self, other: Self) -> Self {
        match(self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                assert(nft_asset1.contract_id == nft_asset2.contract_id);
                // TODO: Remove vecs
                self
            },
            (Asset::TokenAsset(token_asset1), Asset::TokenAsset(token_asset2)) => {
                assert(token_asset1.contract_id == token_asset2.contract_id);
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
