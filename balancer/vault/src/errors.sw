library errors;

pub enum SwapError {
    UnknownAmountInFirstSwap: (),
    CannotSwapSameToken: (),
    SwapLimit: (),
    SwapDeadline: (),
    MalconstructedMultihopSwap: (),
}

pub enum InputError {
    OutOfBounds: (),
    UnsortedArray: (),
    UnsortedTokens: (),
    InputLengthMismatch: (),
    ZeroToken: (),
}

pub enum PoolError {
    MalconstructedMultihopSwp: (),
    InsufficientEth: (),
    InvalidPoolId: (),
    InvalidEthInternalBalance: (),
    InsufficientInternalBalance: (),
    NonZeroTokenBalance: (),
    TokensLengthMustBe2: (),
    InvalidToken: (),
    TokenAlreadyRegistered: (),
    TokensAlreadySet: (),
    TokensMismatch: (),
    PoolNoTokens: (),
    JoinAboveMax: (),
    ExitBelowMin: (),
    InsufficientFlashLoanBalance: (),
    TokenNotRegistered: (),
    SenderNotAssetManager: (),
    CallerNotVault: (),
    InvalidPostLoanBalance: (),
    InsufficientFlashLoanFeeAmount: (),
    CannotUseEthSentinel: (),
    BalanceTotalOverflow: (),
}
