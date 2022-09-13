library bad_documentation;

// This is bad. It's just repeating what I can already read from the field names
pub struct Item1 {
   /// Identifier
   id: u64,

   /// Quantity
   quantity: u64,
}

// This is better. It tells me the context of what the fields are
pub struct Item2 {
   /// Unique identifier used to retrieve the item from a vector of items held in storage
   id: u64,

   /// The number of remaining items left in production
   quantity: u64,
}
