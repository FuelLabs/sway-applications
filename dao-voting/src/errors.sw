library errors;

pub enum Error {
    CannotReinitialize: (),
    NotInitialized: (),
    NotGovernanceToken: (),
    PeriodCannotBeZero: (),
    VoteAmountCannotBeZero: (),
    TokenAmountCanontBeZero: (),
    ApprovalPercentageCannotBeZero: (),
    ApprovalPercentageCannotBeAboveHundred: (),
    NoAssetsSent: (),
    NotEnoughAssets: (),
    InvalidId: (),
    ProposalExpired: (),
    ProposalActive: (),
    ApprovalPercentageNotMet: (),
}
