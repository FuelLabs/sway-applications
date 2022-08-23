library interface;

use std::identity::Identity;

abi Oracle {
    /// Return the owner (node) of the oracle
    fn owner() -> Identity;

    /// Return price of asset
    #[storage(read)] fn price() -> u64;

    /// Set price
    ///
    /// - `price` - New price of tracked asset
    ///
    /// # Reverts
    ///
    /// * When the message sender is not the owner
    #[storage(read, write)] fn set_price(price: u64);
}
