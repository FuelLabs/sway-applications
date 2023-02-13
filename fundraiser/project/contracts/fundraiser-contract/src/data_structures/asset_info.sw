library asset_info;

/// Used to track the total amount pledged to an asset
pub struct AssetInfo {
    /// The amount that is currently pledged
    amount: u64,
}

impl AssetInfo {
    pub fn new() -> Self {
        Self { amount: 0 }
    }
}
