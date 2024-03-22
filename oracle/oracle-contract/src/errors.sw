library;

/// Errors related to access control.
pub enum AccessError {
    /// The sender is not the owner of the contract.
    NotOwner: (),
}
