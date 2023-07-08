library;

use ::errors::RegistrationValidityError;

abi NameRegistry {
    /// Extends the duration of ownership for the name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to extend the duration of
    /// * `duration` - The duration to extend by
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the payment is insufficient to cover the cost for the duration
    /// * If the incorrect asset is sent
    #[payable, storage(read, write)]
    fn extend(name: str[8], duration: u64);

    /// Adds an entry into the registry for the given name.
    ///
    /// Assigns the name to the given owner, and resolves the name to the given identity when queried
    ///
    /// # Arguments
    ///
    /// * `name` - The name to register
    /// * `duration` - The duration to register for
    /// * `owner` - The owner of the name, which will be able to control the ownership and the resolving identity of the name
    /// * `identity` - The identity to which the name would resolve to when queried
    ///
    /// # Reverts
    ///
    /// * If the name is in the registry and it has not expired
    /// * If the payment is insufficient to cover the cost for the duration
    /// * If the incorrect asset is sent
    #[payable, storage(read, write)]
    fn register(name: str[8], duration: u64, owner: Identity, identity: Identity);

    /// Adds a new asset as a method of payment
    ///
    /// # Arguments
    ///
    /// * `id` - Asset for payment
    /// * `rate` - Rate of cost for asset
    ///
    /// # Reverts
    ///
    /// * When called by non-owner
    #[storage(write)]
    fn set_asset(id: ContractId, rate: Option<u64>);

    /// Sets the identity to which the name will resolve to
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set the identity for
    /// * `identity` - The identity which the name will resolve to
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the registration has expired
    /// * If the sender is not the owner of the name
    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity);

    /// Changes the owner of the name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to transfer the ownership of
    /// * `new_owner` - The new owner of the name
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the registration has expired
    /// * If the sender is not the owner of the name
    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity);
}

abi Info {
    /// Returns the expiry timestamp of the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to extend the duration of
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the registration has expired
    #[storage(read)]
    fn expiry(name: str[8]) -> Result<u64, RegistrationValidityError>;

    /// Returns the identity which the name resolves to
    ///
    /// # Arguments
    ///
    /// * `name` - The name to which resolve from
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the registration has expired
    #[storage(read)]
    fn identity(name: str[8]) -> Result<Identity, RegistrationValidityError>;

    /// Returns the owner of the name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to which check
    ///
    /// # Reverts
    ///
    /// * If the name is not registered
    /// * If the registration has expired
    #[storage(read)]
    fn owner(name: str[8]) -> Result<Identity, RegistrationValidityError>;

    /// Returns the cost per interval for the asset
    ///
    /// # Arguments
    ///
    /// * `id` - Asset for payment
    #[storage(read)]
    fn rate(id: ContractId) -> Option<u64>;
}
