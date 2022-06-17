library errors;

pub enum Error {
    CannotReinitialize: (),
    NotInitialized: (),
    NotGovernanceToken: (),
    PeriodCannotBeZero: (),
    VoteAmountCannotBeZero: (),
    TokenAmountCanontBeZero: (),
    ApprovalPercentageCannotBeZero: (),
    NoAssetsSent: (),
    NotEnoughAssets: (),
    InvalidId: (),
    ProposalExpired: (),
    ProposalActive: (),
    ApprovalPercentageNotMet: (),
}