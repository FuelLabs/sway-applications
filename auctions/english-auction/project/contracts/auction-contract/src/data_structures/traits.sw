library;

/// A trait for assets that can be transferred between accounts.
pub trait Asset {
    /// Returns the amount of the asset.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of the asset.
    fn amount(self) -> u64;
    /// Returns the AssetId of the asset.
    ///
    /// # Returns
    ///
    /// * [AssetId] - The AssetId of the asset.
    fn asset_id(self) -> ContractId;
}
