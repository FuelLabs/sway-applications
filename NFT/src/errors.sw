library errors;

pub enum AccessError {
    SenderCannotSetAccessControl: (),
    SenderNotAdmin: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
}

pub enum ApprovalError {
    ApproverCannotBeOwner: (),
}

pub enum InitError {
    AdminIsNone: (),
    CannotReinitialize: (),
}

pub enum InputError {
    NotEnoughTokensToMint: (),
    TokenDoesNotExist: (),
    TokenSupplyCannotBeZero: (),
}
