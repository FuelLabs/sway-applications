library;

/// The information logged when a pool is registered.
///
/// ### Fields
///
/// * `asset_pair`: `(ContractId, ContractId)` - The pair of asset identifiers that make up the pool.
/// * `pool`: `ContractId` - The exchange contract identifier that manages the pool which also identifies the pool asset.
pub struct RegisterPoolEvent {
    asset_pair: (ContractId, ContractId),
    pool: ContractId,
}

/// The information logged when an exchange bytecode root is set.
///
/// ### Fields
///
/// * `root`: `b256` - The bytecode root of the valid exchange contract implementation.
pub struct SetExchangeBytecodeRootEvent {
    root: b256,
}
