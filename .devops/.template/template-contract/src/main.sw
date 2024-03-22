contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

// These are example files so they contain no code. Have to use * to import.
// Avoid using * imports in applications. Explicitly import dependencies whenever possible.
use ::data_structures::example::*;
use ::errors::*;
use ::events::*;
use ::interface::Template;
use ::utils::*;

impl Template for Contract {
    fn template() {}
}
