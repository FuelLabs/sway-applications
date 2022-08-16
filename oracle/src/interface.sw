library interface;

use std::identity::Identity;

abi Oracle {
    /// Initialize the oracle with the owner (node)
    ///
    /// # Arguments
    ///
    /// - `owner` - Identity of node that controls the oracle
    #[storage(write)] fn constructor(owner: Identity);

    /// Set price
    ///
    /// - `new_price` - New price of tracked asset
    #[storage(write)] fn set_price(new_price: u64);

    /// Get price
    #[storage(read)] fn price() -> u64;
}
