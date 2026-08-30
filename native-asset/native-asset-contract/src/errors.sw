library;

pub enum AmountError {
    AmountMismatch: (),
}

pub enum MintError {
    MaxMinted: (),
    SubIdNone: (),
}

pub enum SetError {
    ValueAlreadySet: (),
}
