library interface;

use std::{contract_id::ContractId, identity::Identity, token::*};

abi Token {
    // Burn token coins
    #[storage(read)]
    fn burn_coins(burn_amount: u64);
    // Get balance of the contract coins
    fn get_balance() -> u64;
    // Return the mint amount
    #[storage(read)]
    fn get_mint_amount() -> u64;
    // Get balance of a specified token on contract
    fn get_token_balance(asset_id: ContractId) -> u64;
    // Initialize contract
    #[storage(read, write)]
    fn initialize(identity: Identity, mint_amount: u64);
    // Method called from address to mint coins
    #[storage(read, write)]
    fn mint();
    // Mint token coins
    #[storage(read)]
    fn mint_coins(mint_amount: u64);
    // Set mint amount for each address
    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64);
    // Transfer a contract coins to a given output
    #[storage(read)]
    fn transfer_coins(coins: u64, identity: Identity);
    // Transfer a specified token from the contract to a given output
    #[storage(read)]
    fn transfer_token_to_output(asset_id: ContractId, coins: u64, identity: Identity);
}
