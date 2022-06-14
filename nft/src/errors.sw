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
    CannotReinitialize: (),
    NFTNotInitalized: (),
}

pub enum InputError {
    InputAddressCannotBeZero: (),
    NotEnoughTokensToMint: (),
    TokenDoesNotExist: (),
    TokenSupplyCannotBeZero: (),
}
