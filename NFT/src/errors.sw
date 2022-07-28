library errors;

pub enum AccessError {
    SenderCannotSetAccessControl: (),
    SenderNotAdmin: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
}

pub enum InitError {
    AdminIsNone: (),
    CannotReinitialize: (),
}

pub enum InputError {
    ApprovedDoesNotExist: (),
    NotEnoughTokensToMint: (),
    OwnerDoesNotExist: (),
    TokenDoesNotExist: (),
    TokenSupplyCannotBeZero: (),
}
