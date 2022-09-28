library interface;

use std::{contract_id::ContractId, identity::Identity};

abi Asset {
    /// Get balance of the specified asset on contract.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_id ` - identifier for the specified asset
    fn asset_balance(asset_id: ContractId) -> u64;

    /// Get balance of contract coins.
    fn balance() -> u64;

    /// Burn coins.
    /// 
    /// # Arguments
    /// 
    /// - ` burn_amount ` - amount of coins to be burned
    /// 
    /// # Reverts
    /// 
    /// * When the caller is not the owner of contract
    #[storage(read)]
    fn burn_coins(burn_amount: u64);

    /// Initialize the asset contract with the specified `mint_amount` for coins.
    /// 
    /// # Arguments
    /// 
    /// - ` identity ` - owner of contract
    /// - ` mint_amount ` - amount of coins to be minted
    /// 
    /// # Reverts
    /// 
    /// * When the initializer is called more than once
    #[storage(read, write)]
    fn initialize(identity: Identity, mint_amount: u64);

    /// Mint ` mint_amount ` coins and transfer to the caller.
    /// 
    /// # Reverts
    /// 
    /// * When `mint_amount` is 0, i.e., when mint is closed
    /// * When the caller has already minted coins
    #[storage(read, write)]
    fn mint();

    /// Get mint amount of coins.
    #[storage(read)]
    fn mint_amount() -> u64;

    /// Mint ` mint_amount ` coins.
    /// 
    /// # Arguments
    /// 
    /// - ` mint_amount ` - amount of coins to be minted
    /// 
    /// # Reverts
    /// 
    /// * When the caller is not the owner of contract
    #[storage(read)]
    fn mint_coins(mint_amount: u64);

    /// Set new mint amount for coins.
    /// 
    /// # Arguments
    /// 
    /// - ` mint_amount ` - new mint amount
    /// 
    /// # Reverts
    /// 
    /// * When the caller is not the owner of contract
    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64);

    /// Transfer the specified contract asset to the given identity.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_id ` - identifier for the asset to transfer
    /// - ` amount ` - amount of asset to transfer
    /// - ` identity ` - recipient of the transfer
    /// 
    /// # Reverts
    /// 
    /// * When the caller is not the owner of contract
    #[storage(read)]
    fn transfer_asset_to_output(asset_id: ContractId, amount: u64, identity: Identity);

    /// Transfer contract coins to the given identity.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of coins to transfer
    /// - ` identity ` - recipient of the transfer
    /// 
    /// # Reverts
    /// 
    /// * When the caller is not the owner of contract
    #[storage(read)]
    fn transfer_coins(amount: u64, identity: Identity);
}
