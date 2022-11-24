library utils;

dep errors;

use std::revert::require;
use errors::UserError;

pub fn validate_id(id: u64, count: u64) {
    require(id != 0 && id <= count, UserError::InvalidID);
}
