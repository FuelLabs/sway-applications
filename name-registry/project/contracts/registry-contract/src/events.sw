library;

/// Event for when an asset is set for payment
pub struct AssetRateEvent {
    /// Asset used for payment
    id: ContractId,
    /// Rate of cost for asset
    rate: Option<u64>,
}

/// The event for when a name has its resolving identity changed
pub struct IdentityChangedEvent {
    /// The name which has its identity being changed
    name: str[8],
    /// The new identity which the name will resolve to
    new_identity: Identity,
    /// The previous identity which the name resolved to
    previous_identity: Identity,
}

/// The event for when a name is registered by a new owner (includes expired names being re-registered)
pub struct NameRegisteredEvent {
    /// The new expiry for the name
    expiry: u64,
    /// The name being registered
    name: str[8],
    /// The new owner of the name
    owner: Identity,
    /// The identity to which the name resolves to
    identity: Identity,
}

/// The event for when the owner of a name changes
pub struct OwnerChangedEvent {
    /// The name of which the owner is being changed
    name: str[8],
    /// The new owner of the name
    new_owner: Identity,
    /// The previous owner of the name
    previous_owner: Identity,
}

/// The event for when the expiry for a name is extended by paying of additional fees
pub struct RegistrationExtendedEvent {
    /// The duration by which the expiry is extended
    duration: u64,
    /// The name in subject
    name: str[8],
    /// The new expiry of the name
    new_expiry: u64,
}
