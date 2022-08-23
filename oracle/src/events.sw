library events;

use std::option::Option;

pub struct PriceUpdateEvent {
    /// Updated price
    price: u64,
}
