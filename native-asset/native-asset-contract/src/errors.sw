library;

pub enum AmountError {
    AmountMismatch: (),
}

pub enum MintError {
    MaxMinted: (),
}

pub enum SetError {
    ValueAlreadySet: (),
}
