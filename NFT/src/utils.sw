library utils;

dep data_structures;
dep errors;

use data_structures::TokenMetaData;
use errors::InputError;
use std::{assert::require, option::Option};

/// This function is called to ensure that a token's unqiue identifier maps to an existing token.
///
/// # Arguments
///
/// * `meta_data` - The result of checking the contract's storage for a token's unique identifier.
///
/// # Reverts
///
/// * When the `meta_data` provided is `None`.
pub fn token_metadata(meta_data: Option<TokenMetaData>) -> TokenMetaData {
    require(meta_data.is_some(), InputError::TokenDoesNotExist);
    meta_data.unwrap()
}
