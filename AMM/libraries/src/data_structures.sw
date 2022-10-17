library data_structures;

pub struct PoolInfo {
    base_asset_reserve: u64,
    other_asset_reserve: u64,
    total_liquidity: u64,
}

pub struct PreviewAddLiquidityInfo {
    other_asset_amount: u64,
    received_liquidity: u64,
}

pub struct PreviewInfo {
    amount: u64,
    has_liquidity: bool,
}

pub struct RemoveLiquidityInfo {
    base_asset_amount: u64,
    other_asset_amount: u64,
}
