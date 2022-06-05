library errors;

pub enum UserError {
    AlreadyClaimed: (),
    IncorrectAssetSent: (),
    UnauthorizedUser: (),
}

pub enum StateError {
    CannotReinitialize: (),
    CannotUnpledgeSuccessfulCrowdfund: (),
    CrowdFundEnded: (),
    CrowdFundNotSuccessful: (),
}
