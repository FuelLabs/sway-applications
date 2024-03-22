library;

use ::errors::UserError;

/// Ensures that the given id is valid.
///
/// # Arguments
///
/// * `id`: [u64] - The id to validate.
/// * `count`: [u64] - The count to validate against.
///
/// # Reverts
///
/// * When the id is 0.
/// * When the id is higher than the count.
pub fn validate_campaign_id(id: u64, count: u64) {
    require(id != 0 && id <= count, UserError::InvalidID);
}
