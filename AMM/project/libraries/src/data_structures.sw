library;

/// Information for a particular asset.
///
/// ### Fields
///
/// * `id`: `ContractId` - Identifier of asset.
/// * `amount`: `u64` - Amount of asset that can represent reserve amount, deposit amount, withdraw amount and more depending on the context.
pub struct Asset {
    id: ContractId,
    amount: u64,
}

impl Asset {
    /// This function creates a new `Asset`.
    ///
    /// ### Arguments
    ///
    /// * `id`: `ContractId` - The contract id of the asset.
    /// * `amount`: `u64` - The amount of the asset.
    pub fn new(id: ContractId, amount: u64) -> Self {
        Self { id, amount }
    }
}

/// Information for a particular pair of assets.
///
/// ### Fields
///
/// * `a`: `Asset` - One of the assets in the pair.
/// * `b`: `Asset` - One of the assets in the pair.
pub struct AssetPair {
    a: Asset,
    b: Asset,
}

impl AssetPair {
    /// This function creates a new `AssetPair`.
    ///
    /// ### Arguments
    ///
    /// * `a`: `Asset` - One of the assets in the pair.
    /// * `b`: `Asset` - One of the assets in the pair.
    pub fn new(a: Asset, b: Asset) -> Self {
        Self { a, b }
    }

    /// This function returns the contract ids of both assets in the pair.
    pub fn ids(self) -> (ContractId, ContractId) {
        (self.a.id, self.b.id)
    }

    /// This function returns the amounts of both assets in the pair.
    pub fn amounts(self) -> (u64, u64) {
        (self.a.amount, self.b.amount)
    }

    /// This function returns the Asset with the contract id that matches `this_asset`.
    ///
    /// ### Arguments
    ///
    /// * `this_asset`: `ContractId` - contract id to match with.
    pub fn this_asset(self, this_asset: ContractId) -> Asset {
        if this_asset == self.a.id {
            self.a
        } else {
            self.b
        }
    }

    /// This function returns the Asset with the contract id that does not match `this_asset`.
    ///
    /// ### Arguments
    ///
    /// * `this_asset`: `ContractId` - contract id to match with.
    pub fn other_asset(self, this_asset: ContractId) -> Asset {
        if this_asset == self.a.id {
            self.b
        } else {
            self.a
        }
    }

    /// This function returns a new `AssetPair` with assets sorted based on the reserves.
    ///
    /// ### Arguments
    ///
    /// * `reserves`: `Self` - The asset pair to sort by.
    pub fn sort(self, reserves: Self) -> Self {
        Self {
            a: if self.a.id == reserves.a.id {
                self.a
            } else {
                self.b
            },
            b: if self.a.id == reserves.a.id {
                self.b
            } else {
                self.a
            },
        }
    }
}

impl core::ops::Add for AssetPair {
    fn add(self, other: Self) -> Self {
        Self {
            a: Asset::new(self.a.id, self.a.amount + other.a.amount),
            b: Asset::new(self.b.id, self.b.amount + other.b.amount),
        }
    }
}

impl core::ops::Subtract for AssetPair {
    fn subtract(self, other: Self) -> Self {
        Self {
            a: Asset::new(self.a.id, self.a.amount - other.a.amount),
            b: Asset::new(self.b.id, self.b.amount - other.b.amount),
        }
    }
}

/// Information about the liquidity for a specific asset pair.
///
/// ### Fields
///
/// * `deposits`: `AssetPair` - The asset pair of the deposits.
/// * `liquidity`: `u64` - The amount of liquidity.
/// * `deadline`: `u64` -  The limit on block height for operation.
pub struct LiquidityParameters {
    deposits: AssetPair,
    liquidity: u64,
    deadline: u64,
}

/// Information about a specific pool.
///
/// ### Fields
///
/// * `reserves`: `AssetPair` - The unique identifiers and reserve amounts of the assets that make up the liquidity pool of the exchange contract.
/// * `liquidity`: `u64` - The amount of liquidity pool asset supply in the exchange contract.
pub struct PoolInfo {
    reserves: AssetPair,
    liquidity: u64,
}

/// Information regarding previewing the adding of liquidity.
///
/// ### Fields
///
/// * `other_asset_to_add`: `Asset` - The asset to be added to keep the ratio of the assets that make up the pool. If the ratio is not yet known, i.e. there is no liquidity, then the amount is 0 for preview purposes.
/// * `liquidity_asset_to_receive`: `Asset` - The liquidity pool asset to be minted and transferred to the sender. If the ratio is not yet known, i.e. there is no liquidity, then the ratio is assumed to be 1 for preview purposes.
pub struct PreviewAddLiquidityInfo {
    other_asset_to_add: Asset,
    liquidity_asset_to_receive: Asset,
}

/// Information regarding previewing a swap.
///
/// ### Fields
///
/// * `other_asset`: `Asset` - Other asset to either input or output for a given swap.
/// * `sufficient_reserve`: `bool` - Whether the output reserve is sufficient for swap.
pub struct PreviewSwapInfo {
    other_asset: Asset,
    sufficient_reserve: bool,
}

/// Information regarding removing liquidity.
///
/// ### Fields
///
/// * `removed_amounts`: `AssetPair` - Pool assets that are removed from the reserves and transferred to the sender.
/// * `burned_liquidity`: `Asset` - The amount of liquidity that is burned.
pub struct RemoveLiquidityInfo {
    removed_amounts: AssetPair,
    burned_liquidity: Asset,
}
