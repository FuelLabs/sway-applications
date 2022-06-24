library errors;

pub enum CampaignError {
    CampaignEnded: (),
    CampaignHasBeenCancelled: (),
    DeadlineNotReached: (),
    TargetReached: (),
    TargetNotReached: (),
}

pub enum CreationError {
    ContractNotInitialized: (),
    DeadlineMustBeInTheFuture: (),
    TargetAmountCannotBeZero: (),
}

pub enum UserError {
    AlreadyClaimed: (),
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidID: (),
    NoSuchCampaign: (),
    UnauthorizedUser: (),
    UserHasNotPledged: (),
}
