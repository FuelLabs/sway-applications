library events;

use libraries::data_structures::{Asset, AssetPair};

pub struct AddLiquidityEvent {
    /// Identifiers and amounts of assets added to reserves
    added_assets: AssetPair,
    /// Identifier and amount of liquidity pool assets minted and transferred to sender
    liquidity: Asset,
}

pub struct DefineAssetPairEvent {
    /// Identifier of one of the assets that make up the pool
    asset_a_id: ContractId,
    /// Identifier of the other asset
    asset_b_id: ContractId,
}

pub struct DepositEvent {
    /// Deposited asset that may be withdrawn of used to add liquidity
    deposited_asset: Asset,
    /// New deposit balance of asset in contract
    new_balance: u64,
}

pub struct RemoveLiquidityEvent {
    /// Identifiers and amounts of assets removed from reserves and transferred to sender
    removed_reserve: AssetPair,
    /// Identifier and amount of burned liquidity pool assets
    burned_liquidity: Asset,
}

pub struct SwapEvent {
    /// Identifier and amount of sold asset
    input: Asset,
    /// Identifier and amount of bought asset
    output: Asset,
}

pub struct WithdrawEvent {
    /// Identifier and amount of withdrawn asset
    withdrawn_asset: Asset,
    /// Remaining deposit balance of asset in contract
    remaining_balance: u64,
}
