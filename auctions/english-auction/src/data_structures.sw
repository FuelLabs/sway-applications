library data_structures;

dep errors;

use errors::AssetError;

pub enum Asset {
    NFTAsset: NFTAsset,
    TokenAsset: TokenAsset,
}

impl Asset {
    pub fn amount(self) -> u64 {
        match self {
            Asset::NFTAsset(nft_asset) => {
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

impl core::ops::Add for Asset {
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (

                Asset::TokenAsset(token_asset1),
                Asset::TokenAsset(token_asset2),
            ) => {
                require(token_asset1.contract_id == token_asset2.contract_id, AssetError::AssetsAreNotTheSame);
                let token = TokenAsset {
                    amount: token_asset1.amount + token_asset2.amount,
                    contract_id: token_asset1.contract_id,
                };
                Asset::TokenAsset(token)
            },
            _ => {
                revert(0);
            },
        }
    }
}

impl core::ops::Eq for Asset {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Asset::NFTAsset(nft_asset1), Asset::NFTAsset(nft_asset2)) => {
                nft_asset1.contract_id == nft_asset2.contract_id
            },
            (

                Asset::TokenAsset(token_asset1),
                Asset::TokenAsset(token_asset2),
            ) => {
                token_asset1.contract_id == token_asset2.contract_id
            },
            _ => {
                revert(0);
            },
        }
    }
}

pub struct Auction {
    /// The asset which will be accepted in return for the selling asset.
    bid_asset: Asset,
    /// The current highest bidder of the auction.
    highest_bidder: Option<Identity>,
    /// The block at which the auction's bidding period should end.
    end_block: u64,
    /// The starting price for the auction.
    initial_price: u64,
    /// The price at which the selling asset may be bought outright.
    reserve_price: Option<u64>,
    /// The asset that is being auctioned off.
    sell_asset: Asset,
    /// The seller of the auction.
    seller: Identity,
    /// The state of the auction describing if it is open or closed.
    state: State,
}

pub struct NFTAsset {
    /// The `ContractId` of the NFT that the struct is representing.
    contract_id: ContractId,
    /// The token id of the NFT that the struct is representing.
    token_id: u64,
}

pub enum State {
    /// The state at which the auction is no longer accepting bids.
    Closed: (),
    /// The state where bids may be placed on an auction.
    Open: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Open, State::Open) => true,
            (State::Closed, State::Closed) => true,
            _ => false,
        }
    }
}

pub struct TokenAsset {
    /// The amount of the native asset that the struct is representing.
    amount: u64,
    /// The `ContractId` of the native asset that the struct is representing.
    contract_id: ContractId,
}
