library errors;

pub enum InitError {
    AlreadyInitialized: (),
    ExchangeContractBytecodeRootInvalid: (),
    ExchangeContractDoesNotMatchPair: (),
    NotInitialized: (),
}
