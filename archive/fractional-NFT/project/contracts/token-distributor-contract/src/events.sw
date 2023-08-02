library;

use ::data_structures::TokenDistribution;

pub struct BuybackEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The price at which a single fractionalized NFT token will be purchased for.
    token_price: u64,
}

pub struct CreateEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The struct which contains all relevant information on the token distribution.
    token_distribution: TokenDistribution,
}

pub struct EndEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
}

pub struct PurchaseEvent {
    /// The number of fractionalized NFT tokens purchased by the `buyer`.
    amount: u64,
    /// The user which has purchased fractionalized NFT tokens.
    buyer: Identity,
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
}

pub struct PurchaseAdminEvent {
    /// The new admin which controls the token distribution.
    admin: Option<Identity>,
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The new reserve price at which admin rights may be purchased.
    reserve: Option<u64>,
}

pub struct ReserveEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The new reserve price at which admin rights may be purchased.
    reserve: Option<u64>,
}

pub struct SellEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The user which has sold their fractionalized NFT tokens.
    seller: Identity,
}

pub struct TokenPriceEvent {
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
    /// The new token price at which fractionalized NFT tokens may be purchased.
    token_price: u64,
}

pub struct WithdrawEvent {
    /// The amount of the `external_asset` which has been withdraw from the contract.
    amount: u64,
    /// The asset which has been withdrawn from the contract.
    external_asset: ContractId,
    /// The token contract that holds the NFT.
    fractional_nft_id: ContractId,
}
