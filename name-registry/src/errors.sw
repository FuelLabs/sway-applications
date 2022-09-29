library errors;

pub enum Errors {
    InsufficientPayment: (),
    SenderNotOwner: (),
    NameNotRegistered: (),
    NameNotExpired: (),
    WrongAssetSent: (),
}