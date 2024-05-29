library;

use libraries::data_structures::{Asset, AssetPair};

/// The information logged when liquidity is added.
pub struct AddLiquidityEvent {
    /// Identifiers and amounts of assets added to reserves.
    pub added_assets: AssetPair,
    /// Identifier and amount of liquidity pool assets minted and transferred to sender.
    pub liquidity: Asset,
}

/// The information about the asset pair logged during initialisation.
pub struct DefineAssetPairEvent {
    /// Identifier of one of the assets that make up the pool.
    pub asset_a_id: AssetId,
    /// Identifier of the other asset.
    pub asset_b_id: AssetId,
}

/// The information logged when a deposit is made.
pub struct DepositEvent {
    /// Deposited asset that may be withdrawn or used to add liquidity.
    pub deposited_asset: Asset,
    /// New deposit balance of asset in contract.
    pub new_balance: u64,
}

/// The information logged when liquidity is removed.
pub struct RemoveLiquidityEvent {
    /// Identifiers and amounts of assets removed from reserves and transferred to sender.
    pub removed_reserve: AssetPair,
    /// Identifier and amount of burned liquidity pool assets.
    pub burned_liquidity: Asset,
}

/// The information logged when an asset swap is made.
pub struct SwapEvent {
    /// Identifier and amount of sold asset.
    pub input: Asset,
    /// Identifier and amount of bought asset.
    pub output: Asset,
}

/// The information logged when a withdraw is made.
pub struct WithdrawEvent {
    /// Identifier and amount of withdrawn asset.
    pub withdrawn_asset: Asset,
    /// Remaining deposit balance of asset in contract.
    pub remaining_balance: u64,
}
