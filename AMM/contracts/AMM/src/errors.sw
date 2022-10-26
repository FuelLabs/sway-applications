library errors;

pub enum InitError {
    AlreadyInitialized: (),
    ExchangeContractBytecodeRootInvalid: (),
    PairDoesNotDefinePool: (),
    NotInitialized: (),
}
