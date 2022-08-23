library events;

use std::option::Option;

pub struct PriceUpdateEvent {
    /// Updated price
    price: Option<u64>,
}
