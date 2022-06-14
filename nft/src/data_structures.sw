library data_structures;

use std::{identity::Identity, option::Option};

pub struct MetaData {
    approved: Option<Identity>,
    owner: Identity,
}
