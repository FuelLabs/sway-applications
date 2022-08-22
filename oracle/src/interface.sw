library interface;

use std::{identity::Identity, option::Option};

abi Oracle {
    /// Initialize the oracle with the owner (node)
    ///
    /// # Arguments
    ///
    /// - `owner` - Identity of node that controls the oracle
    #[storage(read, write)] fn constructor(owner: Identity);

    /// Set price
    ///
    /// - `new_price` - New price of tracked asset
    ///
    /// # Reverts
    ///
    /// * When the message sender is not the owner
    #[storage(read, write)] fn set_price(new_price: u64);

    /// Return the owner (node) of the oracle
    #[storage(read)] fn owner() -> Option<Identity>;

    /// Return price of asset
    #[storage(read)] fn price() -> u64;
}
