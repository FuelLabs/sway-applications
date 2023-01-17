contract;

dep data_structures/example;
dep errors;
dep events;
dep interface;
dep utils;

use errors::*;
use events::*;
use example::*;
use interface::Vault;
use utils::*;

impl Vault for Contract {
    fn template() {}
}
