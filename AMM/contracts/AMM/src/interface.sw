library interface;

use std::contract_id::ContractId;

abi AMM {
    /// Add a (given asset ID, exchange contract ID) pair to storage.
    /// The exchange contract defines the pool that consists of the base asset and the given asset.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_id ` - identifier of the asset
    /// - ` exchange_id ` - identifier of exchange contract
    #[storage(write)]
    fn add_exchange_contract(asset_id: ContractId, exchange_id: ContractId);
    /// For the given asset ID, get the exchange contract, i.e., the pool that consists of the base asset and the given asset.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_id ` - identifier of the asset
    #[storage(read)]
    fn exchange_contract(asset_id: ContractId) -> ContractId;
}
