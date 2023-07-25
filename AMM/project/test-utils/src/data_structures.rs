use super::interface::{Exchange, AMM};
use fuels::{
    prelude::{AssetId, ContractId, WalletUnlocked},
    types::{input::Input, output::Output},
};
use std::collections::HashMap;

const AMOUNT_PER_COIN: u64 = 1_000_000;
const COINS_PER_ASSET: u64 = 100;
const DEADLINE: u64 = 1000;
const DEPOSIT_AMOUNTS: (u64, u64) = (10000, 40000);
const LIQUIDITY: u64 = 20000;
pub const NUMBER_OF_ASSETS: u64 = 5;

pub struct AMMContract {
    pub id: ContractId,
    pub instance: AMM<WalletUnlocked>,
    pub pools: HashMap<(AssetId, AssetId), ExchangeContract>,
}

pub struct ExchangeContract {
    pub bytecode_root: Option<ContractId>,
    pub id: ContractId,
    pub instance: Exchange<WalletUnlocked>,
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
    pub number_of_assets: u64,
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

impl LiquidityParameters {
    pub fn new(amounts: Option<(u64, u64)>, deadline: Option<u64>, liquidity: Option<u64>) -> Self {
        Self {
            amounts: amounts.unwrap_or(DEPOSIT_AMOUNTS),
            deadline: deadline.unwrap_or(DEADLINE),
            liquidity: liquidity.unwrap_or(LIQUIDITY),
        }
    }
}

pub struct SwapParameters {
    pub amount: u64,
    pub route_length: u64,
}

pub struct SwapResult {
    pub actual: u64,
    pub expected: Option<u64>,
}

impl Default for WalletAssetConfiguration {
    fn default() -> Self {
        Self {
            number_of_assets: NUMBER_OF_ASSETS,
            coins_per_asset: COINS_PER_ASSET,
            amount_per_coin: AMOUNT_PER_COIN,
        }
    }
}
