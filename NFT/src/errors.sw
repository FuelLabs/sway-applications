library errors;

pub enum AccessError {
    AccessControlNotSet: (),
    SenderCannotSetAccessControl: (),
    SenderDoesNotHaveAccessControl: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
}

pub enum ApprovalError {
    AddressAlreadyGivenAccess: (),
    AddressAlreadyGivenApproval: (),
    ApproverCannotBeOwner: (),
}

pub enum InitError {
    AccessControlSetAndAdminIsNone: (),
    CannotReinitialize: (),
    NFTNotInitalized: (),
}

pub enum InputError {
    InputAddressCannotBeZero: (),
    NotEnoughTokensToMint: (),
    TokenDoesNotExist: (),
    TokenSupplyCannotBeZero: (),
}
