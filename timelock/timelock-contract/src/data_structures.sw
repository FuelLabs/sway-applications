library;

/// Asset used for transfer during execution.
pub struct Asset {
    /// The quantity of an asset.
    pub amount: u64,
    /// Identifier used to distinguish assets.
    pub id: AssetId,
}

/// Represents the time range in which a transaction may be executed.
pub struct ExecutionRange {
    /// The earliest time a transaction may be executed.
    pub start: u64,
    /// The latest time a transaction may be executed.
    pub end: u64,
}
