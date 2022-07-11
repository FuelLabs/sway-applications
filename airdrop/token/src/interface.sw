library interface;

use std::identity::Identity;

abi Token {
    fn mint_to(amount: u64, to: Identity);
}
