library interface;

use std::{
    address::Address,
    contract_id::ContractId,
    vec::Vec,
};


abi ProtocolFeesCollector {
    fn withdraw_collected_fees(
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        recipient: Address
    );
    #[storage(write)]fn set_swap_fee_percentage(new_swap_fee_percentage: u64);
    #[storage(write)]fn set_flash_loan_fee_percentage(new_flash_loan_fee_percentage: u64);
    #[storage(read)]fn get_swap_fee_percentage() -> u64;
    #[storage(read)]fn get_flash_loan_fee_percentage() -> u64;
    fn get_collected_fee_amounts(tokens: Vec<ContractId>) -> Vec<u64>;
    #[storage(read)]fn calculate_flash_loan_fee_amount(amount: u64) -> u64;
    fn pay_fee_amount(token: ContractId, amount: u64);
}