library;

/// The information logged when a pool is registered.
pub struct RegisterPoolEvent {
    /// The pair of asset identifiers that make up the pool.
    asset_pair: (AssetId, AssetId),
    /// The exchange contract identifier that manages the pool which also identifies the pool asset.
    pool: ContractId,
}

// TODO: Remove this when its automatically implemented
impl AbiEncode for RegisterPoolEvent {
    fn abi_encode(self, ref mut buffer: Buffer) {
        buffer.push(self.asset_pair.0);
        buffer.push(self.asset_pair.1);
        buffer.push(self.pool);
    }
}

/// The information logged when an exchange bytecode root is set.
pub struct SetExchangeBytecodeRootEvent {
    /// The bytecode root of the valid exchange contract implementation.
    root: b256,
}
