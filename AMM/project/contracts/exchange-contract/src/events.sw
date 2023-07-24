library;

use libraries::data_structures::{Asset, AssetPair};

/// The information logged when liquidity is added.
///
/// ### Fields
///
/// * `added_assets`: `AssetPair` - Identifiers and amounts of assets added to reserves.
/// * `liquidity`: `Asset` - Identifier and amount of liquidity pool assets minted and transferred to sender.
pub struct AddLiquidityEvent {
    added_assets: AssetPair,
    liquidity: Asset,
}

/// The information about the asset pair logged during initialisation.
///
/// ### Fields
///
/// * `asset_a_id`: `ContractId` - Identifier of one of the assets that make up the pool.
/// * `asset_b_id`: `ContractId` - Identifier of the other asset.
pub struct DefineAssetPairEvent {
    asset_a_id: ContractId,
    asset_b_id: ContractId,
}

/// The information logged when a deposit is made.
///
/// ### Fields
///
/// * `deposited_asset`: `Asset` - Deposited asset that may be withdrawn or used to add liquidity.
/// * `new_balance`: `u64` - New deposit balance of asset in contract.
pub struct DepositEvent {
    deposited_asset: Asset,
    new_balance: u64,
}

/// The information logged when liquidity is removed.
///
/// ### Fields
///
/// * `removed_reserve`: `AssetPair` -  Identifiers and amounts of assets removed from reserves and transferred to sender.
/// * `burned_liquidity`: `Asset` -  Identifier and amount of burned liquidity pool assets.
pub struct RemoveLiquidityEvent {
    removed_reserve: AssetPair,
    burned_liquidity: Asset,
}

/// The information logged when a token swap is made.
///
/// ### Fields
///
/// * `input`: `Asset` - Identifier and amount of sold asset.
/// * `output`: `Asset` - Identifier and amount of bought asset.
pub struct SwapEvent {
    input: Asset,
    output: Asset,
}

/// The information logged when a withdraw made.
///
/// ### Fields
///
/// * `withdrawn_asset`: `Asset` - Identifier and amount of withdrawn asset.
/// * `remaining_balance`: `u64` - Remaining deposit balance of asset in contract.
pub struct WithdrawEvent {
    withdrawn_asset: Asset,
    remaining_balance: u64,
}
