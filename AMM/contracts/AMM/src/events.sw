library events;

pub struct RegisterPoolEvent {
    /// The pair of asset identifiers that make up the pool
    pair: (ContractId, ContractId),
    /// The exchange contract identifier that manages the pool which also identifies the pool asset
    pool: ContractId,
}

pub struct SetExchangeBytecodeRootEvent {
    /// The bytecode root of the valid exchange contract implementation
    root: b256,
}
