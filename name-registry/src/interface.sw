library interface;

abi NameRegistry {
    /// Returns the expiry timestamp of the given name
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to extend the duration of
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is not registered
    #[storage(read)]
    fn expiry(name: str[8]) -> u64;

    /// Extends the registration duration for the name
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to extend the duration of
    /// * `duration` - The duration to extend by
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is not registered, if the payment is not sufficient for the duration, or if the wrong asset is sent
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64);

    /// Returns the identity which the name resolves to
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to which resolve from
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is not registered
    #[storage(read)]
    fn identity(name: str[8]) -> Identity;

    /// Returns the owner of the name
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to which check
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is not registered
    #[storage(read)]
    fn owner(name: str[8]) -> Identity;

    /// Registers the name and assigns the sender as the owner and identity to resolve to
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to register
    /// * `duration` - The duration to register for
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is registered and not expired yet, if the payment is insufficient, or if the wrong asset is sent
    #[storage(read, write)]
    fn register(name: str[8], duration: u64, owner: Identity, identity: Identity);

    /// Sets the identity to which the name will resolve to
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to set the identity for
    /// * `identity` - The identity which the name will resolve to
    /// 
    /// # Reverts
    /// 
    /// Reverts if the name is registered, or if the sender is not the owner of the name
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
    /// Reverts if the name is registered, or if the sender is not the owner of the name
    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity);
}
