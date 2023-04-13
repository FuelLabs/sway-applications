contract;

dep data_structures/example;
dep errors;
dep events;
dep interface;
dep utils;

// These are example files so they contain no code. Have to use * to import.
// Avoid using * imports in applications. Explicitly import dependencies whenever possible.
use errors::*;
use events::*;
use example::*;
use interface::Template;
use utils::*;

impl Template for Contract {
    fn template() {}
}
