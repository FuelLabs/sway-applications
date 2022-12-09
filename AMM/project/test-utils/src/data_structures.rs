use super::abi::{Exchange, AMM};
use fuels::{
    prelude::*,
    tx::{Input, Output},
};
use std::collections::HashMap;

const DEPOSIT_AMOUNTS: (u64, u64) = (10000, 40000);
const DEADLINE: u64 = 1000;
const LIQUIDITY: u64 = 20000;
const NUM_ASSETS: u64 = 5;
const COINS_PER_ASSET: u64 = 100;
const AMOUNT_PER_COIN: u64 = 1_000_000;

pub struct AMMContract {
    pub id: ContractId,
    pub instance: AMM,
    pub pools: HashMap<(AssetId, AssetId), ExchangeContract>,
}

pub struct ExchangeContract {
    pub bytecode_root: Option<ContractId>,
    pub id: ContractId,
    pub instance: Exchange,
    pub pair: (AssetId, AssetId),
}

pub struct ExchangeContractConfiguration {
    pub pair: (AssetId, AssetId),
    pub compute_bytecode_root: bool,
    pub malicious: bool,
    pub salt: [u8; 32],
}

pub struct LiquidityParameters {
    pub amounts: (u64, u64),
    pub deadline: u64,
    pub liquidity: u64,
}

pub struct TransactionParameters {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

pub struct WalletAssetConfiguration {
    pub num_assets: u64,
    pub coins_per_asset: u64,
    pub amount_per_coin: u64,
}

impl ExchangeContractConfiguration {
    pub fn new(
        pair: Option<(AssetId, AssetId)>,
        compute_bytecode_root: Option<bool>,
        malicious: Option<bool>,
        salt: Option<[u8; 32]>,
    ) -> Self {
        Self {
            pair: pair.unwrap_or_default(),
            compute_bytecode_root: compute_bytecode_root.unwrap_or_default(),
            malicious: malicious.unwrap_or_default(),
            salt: salt.unwrap_or_default(),
        }
    }
}

impl Default for LiquidityParameters {
    fn default() -> Self {
        Self {
            amounts: DEPOSIT_AMOUNTS,
            deadline: DEADLINE,
            liquidity: LIQUIDITY,
        }
    }
}

impl LiquidityParameters {
    pub fn new(amounts: Option<(u64, u64)>, deadline: Option<u64>, liquidity: Option<u64>) -> Self {
        Self {
            amounts: amounts.unwrap_or(DEPOSIT_AMOUNTS),
            deadline: deadline.unwrap_or(DEADLINE),
            liquidity: liquidity.unwrap_or(LIQUIDITY),
        }
    }
}

impl Default for WalletAssetConfiguration {
    fn default() -> Self {
        Self {
            num_assets: NUM_ASSETS,
            coins_per_asset: COINS_PER_ASSET,
            amount_per_coin: AMOUNT_PER_COIN,
        }
    }
}

impl WalletAssetConfiguration {
    pub fn new(
        num_assets: Option<u64>,
        coins_per_asset: Option<u64>,
        amount_per_coin: Option<u64>,
    ) -> Self {
        Self {
            num_assets: num_assets.unwrap_or(NUM_ASSETS),
            coins_per_asset: coins_per_asset.unwrap_or(COINS_PER_ASSET),
            amount_per_coin: amount_per_coin.unwrap_or(AMOUNT_PER_COIN),
        }
    }
}
