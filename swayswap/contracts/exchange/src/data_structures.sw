library data_structures;

pub struct PoolInfo {
    eth_reserve: u64,
    token_reserve: u64,
    lp_token_supply: u64,
}

pub struct PositionInfo {
    eth_amount: u64,
    token_amount: u64,
    eth_reserve: u64,
    token_reserve: u64,
    lp_token_supply: u64,
}

pub struct PreviewAddLiquidityInfo {
    token_amount: u64,
    lp_token_received: u64,
}

pub struct PreviewInfo {
    amount: u64,
    has_liquidity: bool,
}

pub struct RemoveLiquidityInfo {
    eth_amount: u64,
    token_amount: u64,
}
