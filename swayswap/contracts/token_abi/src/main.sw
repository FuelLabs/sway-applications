library token_abi;

use std::{address::Address, contract_id::ContractId, token::*};

abi Token {
    // Initialize contract
    #[storage(read, write)]
    fn initialize(mint_amount: u64, address: Address);
    // Set mint amount for each address
    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64);
    // Get balance of the contract coins
    fn get_balance() -> u64;
    // Return the mint amount
    #[storage(read)]
    fn get_mint_amount() -> u64;
    // Get balance of a specified token on contract
    fn get_token_balance(asset_id: ContractId) -> u64;
    // Mint token coins
    #[storage(read)]
    fn mint_coins(mint_amount: u64);
    // Burn token coins
    #[storage(read)]
    fn burn_coins(burn_amount: u64);
    // Transfer a contract coins to a given output
    #[storage(read)]
    fn transfer_coins(coins: u64, address: Address);
    // Transfer a specified token from the contract to a given output
    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, address: Address);
    // Method called from address to mint coins
    #[storage(read, write)]
    fn mint();
}
