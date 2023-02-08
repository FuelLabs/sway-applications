library utils;

dep errors;

use errors::UserError;

pub fn validate_campaign_id(id: u64, count: u64) {
    require(id != 0 && id <= count, UserError::InvalidID);
}
