library utils;

dep data_structures;
dep errors;

use data_structures::MetaData;
use errors::InputError;

use std::{assert::require, option::Option};

/// This function will take an `Option` of type `MetaData` and returns the `MetaData`. It will
/// panic if the `Option` is `None`.
///
/// # Arguments
///
/// * `meta_data` - The `Option` of type `MetaData` which is to be returned.
///
/// # Reverts
///
/// * When the `meta_data` provided is `None`.
pub fn token_metadata(meta_data: Option<MetaData>) -> MetaData {
    require(meta_data.is_some(), InputError::TokenDoesNotExist);
    meta_data.unwrap()
}
