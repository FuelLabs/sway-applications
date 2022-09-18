library errors;

pub enum StakingRewardsError {
    StakeIncorrectToken: (),
    StakeZero: (),
    WithdrawZero: (),
    CallerIsNotRewardsDistributionContract: (),
    ProvidedRewardTooHigh: (),
    SenderNotOwner: (),
    CannotWithdrawTheStakingToken: (),
    PreviousRewardsPeriodMustBeCompleteBeforeChangingTheDurationForTheNewPeriod: (),
}
