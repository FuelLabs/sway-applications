library errors;

pub enum Errors {
    InsufficientPayment: (),
    NameNotRegistered: (),
    NameExpired: (),
    NameNotExpired: (),
    SenderNotOwner: (),
    WrongAssetSent: (),
}
