library swayswap_abi;

abi SwaySwap {
    // Add exchange contract to the token
    #[storage(write)] fn add_exchange_contract(token_id: ContractId, exchange_id: ContractId);
    // Get exchange contract for desired token
    #[storage(read)] fn get_exchange_contract(token_id: ContractId) -> ContractId;
}