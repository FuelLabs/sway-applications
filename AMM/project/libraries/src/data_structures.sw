library data_structures;

pub struct Asset {
    /// Identifier of asset
    id: ContractId,
    /// Amount of asset that can represent reserve amount, deposit amount, withdraw amount and more depending on the context
    amount: u64,
}

impl Asset {
    pub fn new(id: ContractId, amount: u64) -> Self {
        Self { id, amount }
    }
}

pub struct AssetPair {
    a: Asset,
    b: Asset,
}

impl AssetPair {
    pub fn new(a: Asset, b: Asset) -> Self {
        Self { a, b }
    }

    pub fn ids(self) -> (ContractId, ContractId) {
        (self.a.id, self.b.id)
    }

    pub fn amounts(self) -> (u64, u64) {
        (self.a.amount, self.b.amount)
    }

    pub fn this_asset(self, this_asset: ContractId) -> Asset {
        if this_asset == self.a.id {
            self.a
        } else {
            self.b
        }
    }

    pub fn other_asset(self, this_asset: ContractId) -> Asset {
        if this_asset == self.a.id {
            self.b
        } else {
            self.a
        }
    }

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

pub struct LiquidityParameters {
    deposits: AssetPair,
    liquidity: u64,
    deadline: u64,
}

pub struct PoolInfo {
    /// Unique identifiers and reserve amounts of the assets that make up the liquidity pool of the exchange contract
    reserves: AssetPair,
    /// The amount of liquidity pool asset supply in the exchange contract
    liquidity: u64,
}

pub struct PreviewAddLiquidityInfo {
    /// The asset to be added to keep the ratio of the assets that make up the pool
    /// If the ratio is not yet known, i.e., there is no liquidity, then the amount is 0 for preview purposes
    other_asset_to_add: Asset,
    /// The liquidity pool asset to be minted and transferred to the sender
    /// If the ratio is not yet known, i.e., there is no liquidity, then the ratio is assumed to be 1 for preview purposes
    liquidity_asset_to_receive: Asset,
}

pub struct PreviewSwapInfo {
    /// Other asset to either input or output for a given swap
    other_asset: Asset,
    /// Whether the output reserve is sufficient for swap
    sufficient_reserve: bool,
}

pub struct RemoveLiquidityInfo {
    /// Pool assets that are removed from the reserves and transferred to the sender
    removed_amounts: AssetPair,
    /// The amount of liquidity that is burned
    burned_liquidity: Asset,
}
