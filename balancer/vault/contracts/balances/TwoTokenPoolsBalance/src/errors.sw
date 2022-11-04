library errors;

// errors for the contract
pub enum Error {
    TOKEN_ALREADY_REGISTERED: (),
    UNSORTED_TOKENS: (),
    TOKENS_ALREADY_SET: (),
    NONZERO_TOKEN_BALANCE: (),
}
