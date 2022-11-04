library data_structures;

use std::contract_id::ContractId;

pub const TOTAL_TOKENS = 20;
pub const TOTAL_SUPPLY = 10000000;
pub const ONE = 1; // 18 decimal places
pub const TWO = 2;
pub const FOUR = 4;
pub const MAX_POW_RELATIVE_ERROR = 10000; // 10^(-14)
pub const DEFAULT_MINIMUM_BPT = 1000000;
pub const MAX_POW_RELATIVE_ERROR = 10000; // 10^(-14)
pub const UNHANDLED_JOIN_KIND = 310;
// Invariant growth limit: non-proportional joins cannot cause the invariant to increase by more than this ratio.
pub const MAX_INVARIANT_RATIO = 3;
pub const MIN_SWAP_FEE_PERCENTAGE = 10000000000; // 0.0001%
pub const INVALID_TOKEN = 309;

pub const TOKEN0: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN1: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN2: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN3: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN4: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN5: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN6: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN7: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN8: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN9: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN10: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN11: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN12: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN13: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN14: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN15: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN16: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN17: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN18: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};
pub const TOKEN19: ContractId = ContractId {
    value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b
};

// All token balances are normalized to behave as if the token had 18 decimals. We assume a token's decimals will
// not change throughout its lifetime, and store the corresponding scaling factor for each at pub construction time.
// These factors are always greater than or equal to one: tokens with more than 18 decimals are not supported.

pub const SCALING_FACTOR0 = 10;
pub const SCALING_FACTOR1 = 10;
pub const SCALING_FACTOR2 = 10;
pub const SCALING_FACTOR3 = 10;
pub const SCALING_FACTOR4 = 10;
pub const SCALING_FACTOR5 = 10;
pub const SCALING_FACTOR6 = 10;
pub const SCALING_FACTOR7 = 10;
pub const SCALING_FACTOR8 = 10;
pub const SCALING_FACTOR9 = 10;
pub const SCALING_FACTOR10 = 10;
pub const SCALING_FACTOR11 = 10;
pub const SCALING_FACTOR12 = 10;
pub const SCALING_FACTOR13 = 10;
pub const SCALING_FACTOR14 = 10;
pub const SCALING_FACTOR15 = 10;
pub const SCALING_FACTOR16 = 10;
pub const SCALING_FACTOR17 = 10;
pub const SCALING_FACTOR18 = 10;
pub const SCALING_FACTOR19 = 10;

pub const NORMALIZED_WEIGHT0 = 10;
pub const NORMALIZED_WEIGHT1 = 10;
pub const NORMALIZED_WEIGHT2 = 10;
pub const NORMALIZED_WEIGHT3 = 10;
pub const NORMALIZED_WEIGHT4 = 10;
pub const NORMALIZED_WEIGHT5 = 10;
pub const NORMALIZED_WEIGHT6 = 10;
pub const NORMALIZED_WEIGHT7 = 10;
pub const NORMALIZED_WEIGHT8 = 10;
pub const NORMALIZED_WEIGHT9 = 10;
pub const NORMALIZED_WEIGHT10 = 10;
pub const NORMALIZED_WEIGHT11 = 10;
pub const NORMALIZED_WEIGHT12 = 10;
pub const NORMALIZED_WEIGHT13 = 10;
pub const NORMALIZED_WEIGHT14 = 10;
pub const NORMALIZED_WEIGHT15 = 10;
pub const NORMALIZED_WEIGHT16 = 10;
pub const NORMALIZED_WEIGHT17 = 10;
pub const NORMALIZED_WEIGHT18 = 10;
pub const NORMALIZED_WEIGHT19 = 10;

// In order to preserve backwards compatibility, make sure new join and exit kinds are added at the end of the enum.
pub enum JoinKind {
    INIT: (),
    EXACT_TOKENS_IN_FOR_BPT_OUT: (),
    TOKEN_IN_FOR_EXACT_BPT_OUT: (),
    ALL_TOKENS_IN_FOR_EXACT_BPT_OUT: (),
    ADD_TOKEN: (),
    // for Managed Pool
}

pub enum ExitKind {
    EXACT_BPT_IN_FOR_ONE_TOKEN_OUT: (),
    EXACT_BPT_IN_FOR_TOKENS_OUT: (),
    BPT_IN_FOR_EXACT_TOKENS_OUT: (),
    REMOVE_TOKEN: (),
    // for ManagedPool
}
