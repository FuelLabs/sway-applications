library;

abi Oracle {
    /// Return the owner (node) of the oracle.
    ///
    /// # Additional Information
    ///
    /// The owner is initialized to the first deterministically generated wallet using the SDK.
    ///
    /// # Returns
    ///
    /// * [Identity] - The owner of the oracle.
    fn owner() -> Identity;

    /// Return price of asset.
    ///
    /// # Additional Information
    ///
    /// The price is `None` until the price is set for the first time.
    /// # Returns
    ///
    /// * [Option<u64>] - The price of the tracked asset.
    #[storage(read)]
    fn price() -> Option<u64>;

    /// Changes the price in storage to the value of `price`.
    ///
    /// # Arguments
    ///
    /// * `price`: [u64] - New price of tracked asset.
    ///
    /// # Reverts
    ///
    /// * When the message sender is not the owner.
    #[storage(write)]
    fn set_price(price: u64);
}
