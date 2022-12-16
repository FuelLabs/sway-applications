library events;

dep data_structures;

use data_structures::TokenDistribution;

pub struct Buyback {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The price at which a single fractionalized NFT token will be purchased for.
    token_price: u64,
}

pub struct Canceled {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
}

pub struct Closed {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
}

pub struct Created {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The struct which contains all relevant information on the token distribution.
    token_distribution: TokenDistribution,
}

pub struct Purchased {
    /// The number of fractionalized NFT tokens purchased by the `buyer`.
    amount: u64,
    /// The user which has purchased fractionalized NFT tokens.
    buyer: Identity,
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
}

pub struct PurchasedOwnership {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The new owner which controls the token distribution.
    owner: Option<Identity>,
    /// The new reserve price at which ownership may be purchased.
    reserve: Option<u64>,
}

pub struct Sell {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The user which has sold their fractionalized NFT tokens.
    seller: Identity,
}

pub struct Reserve {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The new reserve price at which ownership may be purchased.
    reserve: Option<u64>,
}

pub struct TokenPrice {
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
    /// The new token price at which fractionalized NFT tokens may be purchased.
    token_price: u64,
}

pub struct Withdraw {
    /// The amount of the `external_asset` which has been withdraw from the contract.
    amount: u64,
    /// The asset which has been withdrawn from the contract.
    external_asset: ContractId,
    /// The token contract that holds the NFT.
    fractional_nft: ContractId,
}
