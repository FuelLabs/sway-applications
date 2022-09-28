library interface;

use std::contract_id::ContractId;

abi AMM {
    /// Add exchange contract to given token.
    /// 
    /// # Arguments
    /// 
    /// - ` exchange_id ` - identifier of exchange contract
    /// - ` token_id ` - identifier of token contract
    #[storage(write)]
    fn add_exchange_contract(exchange_id: ContractId, token_id: ContractId);
    /// Get exchange contract for desired token.
    /// 
    /// # Arguments
    /// 
    /// - ` token_id ` - identifier of token contract
    #[storage(read)]
    fn exchange_contract(token_id: ContractId) -> ContractId;
}
