library errors;

pub enum Error {
    InvalidToken: (),
    Uninitialized: (),
    InputLengthMismatch: (),
    ZeroInvariant: (),
    MaxPowRelativeError: (),
    MulOverflow: (),
    MinimumBpt: (),
    ZeroDivision: (),
    DivInternal: (),
    BptOutMinAmount: (),
    OutOfBounds: (),
    MaxOutBptForTokenIn: (),
    MinSwapFeePercentage: (),
    MaxSwapFeePercentage: (),
    BptInMaxAmount: (),
    MinBptInForTokenOut: (),
}


