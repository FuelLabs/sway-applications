library;

/// Errors related to creation of a proposal.
pub enum CreationError {
    /// The proposal duration cannot be zero.
    DurationCannotBeZero: (),
    /// The acceptance percentage is outside the valid range of 0 to 100.
    InvalidAcceptancePercentage: (),
}

/// Errors related to initialization of the contract.
pub enum InitializationError {
    /// The contract has already been initialized.
    CannotReinitialize: (),
    /// The contract has not been initialized.
    ContractNotInitialized: (),
}

/// Errors related to proposal execution.
pub enum ProposalError {
    /// The proposal did not recieve enough approvals.
    InsufficientApprovals: (),
    /// The proposal has already been executed.
    ProposalExecuted: (),
    /// The proposal has expired.
    ProposalExpired: (),
    /// The proposal is still active.
    ProposalStillActive: (),
}

/// Errors related to user actions.
pub enum UserError {
    /// Deposit or withdrawal amounts cannot be zero.
    AmountCannotBeZero: (),
    /// The incorrect asset type was sent.
    IncorrectAssetSent: (),
    /// The user does not have enough balance to perform the action.
    InsufficientBalance: (),
    /// The proposal for the given id does not exist.
    InvalidId: (),
    /// Cannot vote with zero amount.
    VoteAmountCannotBeZero: (),
}
