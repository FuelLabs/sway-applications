library utils;

use std::{
    address::Address,
    vec::Vec,
    option::Option,
};

// helping function
pub fn vec_contains(vec: Vec<Address>, search: Address) -> bool {
    let mut count = 0;
    while(count < vec.len()) {
        if vec.get(count).unwrap() == search {
            return true;
        }
        count = count + 1;
    }

    return false;
}