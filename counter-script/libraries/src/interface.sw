library;

abi Counter {
    /// Increments the counter by one.
    ///
    /// # Returns
    ///
    /// * [u64] - The new value of the counter.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Write: `1`
    #[storage(read, write)]
    fn increment() -> u64;
    /// Returns the current value of the counter.
    ///
    /// # Returns
    ///
    /// * [u64] - The current value of the counter.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn count() -> u64;
    /// Clears the counter.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Clears: `1`
    #[storage(write)]
    fn clear();
}
