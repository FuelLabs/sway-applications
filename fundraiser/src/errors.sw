library errors;

pub enum Error {
    NoSuchCampaign: (),
}

pub enum StateError {
    CannotReinitialize: (),
    CannotUnpledgeSuccessfulFundraise: (),
    FundraiseEnded: (),
    FundraiseNotSuccessful: (),
}

pub enum UserError {
    AlreadyClaimed: (),
    IncorrectAssetSent: (),
    UnauthorizedUser: (),
}
