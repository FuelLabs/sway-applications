library errors;

pub enum LinearPoolErrors {
    OUT_OF_BOUNDS: (),
    INVALID_TOKEN: (),
    UNHANDLED_BY_LINEAR_POOL: (),
    LOWER_GREATER_THAN_UPPER_TARGET: (),
    UPPER_TARGET_TOO_HIGH: (),
    OUT_OF_TARGET_RANGE: (),
    OUT_OF_NEW_TARGET_RANGE: (),
    INVALID_INITIALIZATION: (),
    MINIMUM_BPT: (),
}