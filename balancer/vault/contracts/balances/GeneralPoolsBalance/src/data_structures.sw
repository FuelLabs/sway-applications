library data_structures;

use std::contract_id::ContractId;

pub const TOKEN_ALREADY_REGISTERED = 512;

pub struct IERC20ToBytes32MapEntry {
    _key: (b256, ContractId),
    _value: b256,
}

pub struct IERC20ToBytes32Map {
    // Number of entries in the map
    length: u64,
    // Storage of map keys and values
    entries: u64,
    // Position of the entry defined by a key in the `entries` array, plus 1
    // because index 0 means a key is not in the map.
    indexes: ContractId,
}