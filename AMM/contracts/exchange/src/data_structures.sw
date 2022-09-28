library data_structures;

pub struct PoolInfo {
    eth_reserve: u64,
    lp_token_supply: u64,
    token_reserve: u64,
}

pub struct PreviewAddLiquidityInfo {
    lp_token_received: u64,
    token_amount: u64,
}

pub struct PreviewInfo {
    amount: u64,
    has_liquidity: bool,
}

pub struct RemoveLiquidityInfo {
    eth_amount: u64,
    token_amount: u64,
}
