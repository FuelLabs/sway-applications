library errors;

pub enum Errors {
    InsufficientPayment: (),
    NameNotRegistered: (),
    NameNotExpired: (),
    SenderNotOwner: (),
    WrongAssetSent: (),
}