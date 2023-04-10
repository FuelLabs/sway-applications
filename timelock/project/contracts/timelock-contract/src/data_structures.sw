library;

/// Asset used for transfer during execution
pub struct Asset {
    /// The quantity of an asset
    amount: u64,
    /// Identifier used to distinguish assets
    id: ContractId,
}

/// Represents the time range in which a transaction may be executed
pub struct ExecutionRange {
    /// The earliest time a transaction may be executed
    start: u64,
    /// The latest time a transaction may be executed
    end: u64,
}
