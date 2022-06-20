library errors;

pub enum CampaignError {
    CampaignEnded: (),
    CampaignHasBeenCancelled: (),
    DeadlineNotReached: (),
    TargetReached: (),
    TargetNotReached: (),
}

pub enum CreationError {
    CannotUseBaseAsset: (),
    ContractNotInitialized: (),
    DeadlineMustBeInTheFuture: (),
    TargetAmountCannotBeZero: (),
}

pub enum UserError {
    AlreadyClaimed: (),
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidHistoryId: (),
    NoSuchCampaign: (),
    UnauthorizedUser: (),
    UserHasNotPledged: (),
}
