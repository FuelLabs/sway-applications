library;

use ::errors::UserError;

pub fn validate_id(id: u64, count: u64) {
    require(id < count, UserError::InvalidId);
}
