library events;

use std::contract_id::ContractId;

pub struct DefineValidExchangeEvent {
    /// The bytecode root of the valid exchange contract implementation
    root: b256,
}

pub struct RegisterPoolEvent {
    /// The pair of asset identifiers that make up the pool
    pair: (ContractId, ContractId),
    /// The exchange contract identifier that manages the pool which also identifies the pool asset
    pool: ContractId,
}
