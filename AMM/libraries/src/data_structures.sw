library data_structures;

use std::contract_id::ContractId;

pub struct PoolInfo {
    /// Unique identifier that makes up one side of the liquidity pool in an exchange contract
    asset_a_id: ContractId,
    /// Unique identifier that makes up the other side of the liquidity pool in an exchange contract
    asset_b_id: ContractId,
    /// The amount of asset a reserve in the exchange contract
    asset_a_reserve: u64,
    /// The amount of asset b reserve in the exchange contract
    asset_b_reserve: u64,
    /// The amount of liquidity pool asset supply in the exchange contract
    liquidity: u64,
}

pub struct PreviewAddLiquidityInfo {
    /// The amount of other asset to be added to keep the ratio of the assets that make up the pool
    /// If the ratio is not yet known, i.e., there is no liquidity, then the amount is 0 for preview purposes
    other_asset_amount_to_add: u64,
    /// The amount of liquidity pool assets to be minted and transferred to the sender
    /// If the ratio is not yet known, i.e., there is no liquidity, then the ratio is assumed to be 1 for preview purposes
    liquidity_asset_amount_to_receive: u64,
}

pub struct PreviewSwapInfo {
    /// The amount of other asset to either input or output for a given swap
    amount: u64,
    /// Whether the reserve of the queried asset is depleted after the swap
    reserve_depleted: bool,
}

pub struct RemoveLiquidityInfo {
    /// The amount of asset a that is removed from the reserves and transferred to the sender
    asset_a_amount: u64,
    /// The amount of asset b that is removed from the reserves and transferred to the sender
    asset_b_amount: u64,
    /// The amount of liquidity that is burned
    liquidity: u64,
}
