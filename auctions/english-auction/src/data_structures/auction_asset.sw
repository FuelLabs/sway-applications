library auction_asset;

dep nft_asset;
dep token_asset;
dep traits;

use nft_asset::NFTAsset;
use token_asset::TokenAsset;
use traits::Asset;

pub enum AuctionAsset {
    NFTAsset: NFTAsset,
    TokenAsset: TokenAsset,
}

impl Asset for AuctionAsset {
    fn amount(self) -> u64 {
        match self {
            AuctionAsset::NFTAsset(nft_asset) => {
                nft_asset.amount()
            },
            AuctionAsset::TokenAsset(token_asset) => {
                token_asset.amount()
            },
        }
    }

    fn contract_id(self) -> ContractId {
        match self {
            AuctionAsset::NFTAsset(nft_asset) => {
                nft_asset.contract_id()
            },
            AuctionAsset::TokenAsset(token_asset) => {
                token_asset.contract_id()
            },
        }
    }
}

impl core::ops::Add for AuctionAsset {
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (AuctionAsset::TokenAsset(a1), AuctionAsset::TokenAsset(a2)) => {
                AuctionAsset::TokenAsset(a1 + a2)
            },
            (AuctionAsset::NFTAsset(a1), AuctionAsset::NFTAsset(a2)) => {
                AuctionAsset::NFTAsset(a1 + a2)
            }
            _ => {
                revert(0);
            },
        }
    }
}

impl core::ops::Eq for AuctionAsset {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (AuctionAsset::NFTAsset(a1), AuctionAsset::NFTAsset(a2), ) => {
                a1 == a2
            },
            (AuctionAsset::TokenAsset(a1), AuctionAsset::TokenAsset(a2), ) => {
                a1 == a2
            },
            _ => {
                false
            },
        }
    }
}

impl core::ops::Ord for AuctionAsset {
    fn gt(self, other: Self) -> bool {
        match (self, other) {
            (AuctionAsset::NFTAsset(a1), AuctionAsset::NFTAsset(a2), ) => {
                a1 > a2
            },
            (AuctionAsset::TokenAsset(a1), AuctionAsset::TokenAsset(a2), ) => {
                a1 > a2
            },
            _ => {
                revert(0);
            },
        }
    }

    fn lt(self, other: Self) -> bool {
        match (self, other) {
            (AuctionAsset::NFTAsset(a1), AuctionAsset::NFTAsset(a2), ) => {
                a1 < a2
            },
            (AuctionAsset::TokenAsset(a1), AuctionAsset::TokenAsset(a2), ) => {
                a1 < a2
            },
            _ => {
                revert(0);
            },
        }
    }
}
