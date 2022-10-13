library errors;

pub enum AssetErrors {
    InsufficientPayment: (),
    IncorrectAssetSent: (),
}

pub enum AuthorisationErrors {
    SenderNotOwner: (),
}

pub enum ValidityErrors {
    NameNotRegistered: (),
    NameExpired: (),
    NameNotExpired: (),
}
