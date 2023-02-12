library errors;

pub enum CampaignError {
    CampaignEnded: (),
    CampaignHasBeenCancelled: (),
    DeadlineNotReached: (),
    TargetNotReached: (),
}

pub enum CreationError {
    DeadlineMustBeInTheFuture: (),
    TargetAmountCannotBeZero: (),
}

pub enum UserError {
    AlreadyClaimed: (),
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidID: (),
    UnauthorizedUser: (),
    UserHasNotPledged: (),
}
