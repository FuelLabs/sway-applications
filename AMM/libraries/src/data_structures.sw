library;

/// Information for a particular asset.
pub struct Asset {
    /// Identifier of asset.
    pub id: AssetId,
    /// Amount of asset that can represent reserve amount, deposit amount, withdraw amount and more depending on the context.
    pub amount: u64,
}

impl Asset {
    /// This function creates a new `Asset`.
    ///
    /// # Arguments
    ///
    /// * `id`: [AssetId] - The AssetId of the asset.
    /// * `amount`: [u64] - The amount of the asset.
    ///
    /// # Returns
    ///
    /// * `Asset` - The new asset.
    pub fn new(id: AssetId, amount: u64) -> Self {
        Self { id, amount }
    }
}

/// Information for a particular pair of assets.
pub struct AssetPair {
    /// One of the assets in the pair.
    pub a: Asset,
    /// One of the assets in the pair.
    pub b: Asset,
}

impl AssetPair {
    /// This function creates a new `AssetPair`.
    ///
    /// # Arguments
    ///
    /// * `a`: [Asset] - One of the assets in the pair.
    /// * `b`: [Asset] - One of the assets in the pair.
    ///
    /// # Returns
    ///
    /// * `AssetPair` - The new asset pair.
    pub fn new(a: Asset, b: Asset) -> Self {
        Self { a, b }
    }

    /// This function returns the contract ids of both assets in the pair.
    ///
    /// # Returns
    ///
    /// * `(AssetId, AssetId)` - The contract ids of both assets in the pair.
    pub fn ids(self) -> (AssetId, AssetId) {
        (self.a.id, self.b.id)
    }

    /// This function returns the amounts of both assets in the pair.
    ///
    /// # Returns
    ///
    /// * `(u64, u64)` - The amounts of both assets in the pair.
    pub fn amounts(self) -> (u64, u64) {
        (self.a.amount, self.b.amount)
    }

    /// This function returns the Asset with the AssetId that matches `this_asset`.
    ///
    /// # Arguments
    ///
    /// * `this_asset`: [AssetId] - AssetId to match with.
    ///
    /// # Returns
    ///
    /// * `Asset` - The AssetId that matches `this_asset`.
    pub fn this_asset(self, this_asset: AssetId) -> Asset {
        if this_asset == self.a.id {
            self.a
        } else {
            self.b
        }
    }

    /// This function returns the Asset with the AssetId that does not match `this_asset`.
    ///
    /// # Arguments
    ///
    /// * `this_asset`: [AssetId] - AssetId to match with.
    ///
    /// # Returns
    ///
    /// * `Asset` - The AssetId that does not match `this_asset`.
    pub fn other_asset(self, this_asset: AssetId) -> Asset {
        if this_asset == self.a.id {
            self.b
        } else {
            self.a
        }
    }

    /// This function returns a new `AssetPair` with assets sorted based on the reserves.
    ///
    /// # Arguments
    ///
    /// * `reserves`: [Self] - The asset pair to sort by.
    ///
    /// # Returns
    ///
    /// * `AssetPair` - The new asset pair with assets sorted based on the reserves.
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
pub struct LiquidityParameters {
    /// The asset pair of the deposits.
    pub deposits: AssetPair,
    /// The amount of liquidity.
    pub liquidity: u64,
    /// The limit on block height for operation.
    pub deadline: u64,
}

/// Information about a specific pool.
pub struct PoolInfo {
    /// The unique identifiers and reserve amounts of the assets that make up the liquidity pool of the exchange contract.
    pub reserves: AssetPair,
    /// The amount of liquidity pool asset supply in the exchange contract.
    pub liquidity: u64,
}

/// Information regarding previewing the adding of liquidity.
pub struct PreviewAddLiquidityInfo {
    /// The asset to be added to keep the ratio of the assets that make up the pool. If the ratio is not yet known, i.e. there is no liquidity, then the amount is 0 for preview purposes.
    pub other_asset_to_add: Asset,
    /// The liquidity pool asset to be minted and transferred to the sender. If the ratio is not yet known, i.e. there is no liquidity, then the ratio is assumed to be 1 for preview purposes.
    pub liquidity_asset_to_receive: Asset,
}

/// Information regarding previewing a swap.
pub struct PreviewSwapInfo {
    /// Other asset to either input or output for a given swap.
    pub other_asset: Asset,
    /// Whether the output reserve is sufficient for swap.
    pub sufficient_reserve: bool,
}

/// Information regarding removing liquidity.
pub struct RemoveLiquidityInfo {
    /// Pool assets that are removed from the reserves and transferred to the sender.
    pub removed_amounts: AssetPair,
    /// The amount of liquidity that is burned.
    pub burned_liquidity: Asset,
}
