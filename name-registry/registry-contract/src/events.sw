library;

use std::string::String;

/// Event for when an asset is set for payment
pub struct AssetRateEvent {
    /// Asset used for payment
    pub asset: AssetId,
    /// Rate of cost for asset
    pub rate: Option<u64>,
}

/// The event for when a name has its resolving identity changed
pub struct IdentityChangedEvent {
    /// The name which has its identity being changed
    pub name: String,
    /// The new identity which the name will resolve to
    pub new_identity: Identity,
    /// The previous identity which the name resolved to
    pub previous_identity: Identity,
}

/// The event for when a name is registered by a new owner (includes expired names being re-registered)
pub struct NameRegisteredEvent {
    /// The new expiry for the name
    pub expiry: u64,
    /// The name being registered
    pub name: String,
    /// The new owner of the name
    pub owner: Identity,
    /// The identity to which the name resolves to
    pub identity: Identity,
}

/// The event for when the owner of a name changes
pub struct OwnerChangedEvent {
    /// The name of which the owner is being changed
    pub name: String,
    /// The new owner of the name
    pub new_owner: Identity,
    /// The previous owner of the name
    pub previous_owner: Identity,
}

/// The event for when the expiry for a name is extended by paying of additional fees
pub struct RegistrationExtendedEvent {
    /// The duration by which the expiry is extended
    pub duration: u64,
    /// The name in subject
    pub name: String,
    /// The new expiry of the name
    pub new_expiry: u64,
}
