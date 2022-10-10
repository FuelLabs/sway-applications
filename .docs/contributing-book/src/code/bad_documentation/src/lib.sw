library bad_documentation;

// ANCHOR: data_structures
// This is bad. It's repeating the names of the fields which can be easily read
pub struct Item1 {
    /// Identifier
    id: u64,
    /// Quantity
    quantity: u64,
}

// This is better. It conveys the context of what the fields are
pub struct Item2 {
    /// Unique identifier used to retrieve the item from a vector of items held in storage
    id: u64,
    /// The number of remaining items left in production
    quantity: u64,
}
// ANCHOR_END: data_structures
