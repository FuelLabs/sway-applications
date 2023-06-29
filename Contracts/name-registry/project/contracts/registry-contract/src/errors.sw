library;

pub enum AssetError {
    InsufficientPayment: (),
    IncorrectAssetSent: (),
}

pub enum AuthorisationError {
    SenderNotOwner: (),
}

pub enum RegistrationValidityError {
    NameNotRegistered: (),
    NameExpired: (),
    NameNotExpired: (),
}
