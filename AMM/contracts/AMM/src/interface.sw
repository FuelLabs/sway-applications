library interface;

use std::contract_id::ContractId;

abi AMM {
    // Add exchange contract to the token
    #[storage(write)]
    fn add_exchange_contract(exchange_id: ContractId, token_id: ContractId);
    // Get exchange contract for desired token
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId;
}
