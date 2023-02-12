library data_structures;

/// Represents the time range in which a transaction may be executed
pub struct ExecutionRange {
    /// The earliest time a transaction may be executed
    start: u64,
    /// The latest time a transaction may be executed
    end: u64,
}
