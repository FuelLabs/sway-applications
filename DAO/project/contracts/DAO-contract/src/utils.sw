library;

use ::errors::UserError;

/// Validates that the given id is within the given count.
///
/// # Arguments
///
/// * `id`: [u64] - The id to validate.
/// * `count`: [u64] - The count to validate against.
///
/// # Reverts
///
/// * When the id is greater than the count.
pub fn validate_id(id: u64, count: u64) {
    require(id < count, UserError::InvalidId);
}
