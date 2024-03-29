library;

pub enum AssetError {
    /// Emitted when the amount of asset sent is less than the fee.
    InsufficientPayment: (),
    /// Emitted when an incorrect asset is sent for payment.
    IncorrectAssetSent: (),
}

pub enum AuthorizationError {
    /// Emitted when the caller is not the owner of a record or the registry.
    SenderNotOwner: (),
}

pub enum RegistrationValidityError {
    /// Emitted when interacting with a name that has expired.
    NameExpired: (),
    /// Emitted when interacting with a name that has never been registered.
    NameNotRegistered: (),
    /// Emitted when attempting to register a name that has not expired.
    NameNotExpired: (),
    /// Emitted when the name length is less than 3 bytes.
    NameTooShort: (),
}
