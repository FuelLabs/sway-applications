contract;

dep errors;
dep events;
dep interface;

use errors::Error;
use interface::ProtocolFeesCollector;
use events::{
    SwapFeePercentageChanged, 
    FlashLoanFeePercentageChanged
};


use std::{
    contract_id::ContractId,
    address::Address,
    option::Option,
    assert::assert ,
    vec::Vec,
    context::balance_of,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::*,
    revert::{revert, require},
    context::call_frames::contract_id,
    token::transfer_to_output,
    logging::log,
};

use InputHelpers::ensure_input_length_match;
use FixedPoint::mul_up;

storage {
    swap_fee_percentage: u64 = 1000000,
    flash_loan_fee_percentage: u64 = 1000000,
}


// Absolute maximum fee percentages (1e18 = 100%, 1e16 = 1%).
const _MAX_PROTOCOL_SWAP_FEE_PERCENTAGE = 50000000000000000;
const _MAX_PROTOCOL_FLASH_LOAN_FEE_PERCENTAGE = 10000000000000000;


// All fee percentages are 18-decimal fixed point numbers.
// The swap fee is charged whenever a swap occurs, as a percentage of the fee charged by the Pool. These are not
// actually charged on each individual swap: the `Vault` relies on the Pools being honest and reporting fees due
// when users join and exit them.
// The flash loan fee is charged whenever a flash loan occurs, as a percentage of the tokens lent.


// @dev This an auxiliary contract to the Vault, deployed by it during construction. It offloads some of the tasks the
// Vault performs to reduce its overall bytecode size.
//
// The current values for all protocol fee percentages are stored here, and any tokens charged as protocol fees are
// sent to this contract, where they may be withdrawn by authorized entities. All authorization tasks are delegated
// to the Vault's own authorizer.
impl ProtocolFeesCollector for Contract {
    fn withdraw_collected_fees(
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        recipient: Address
    ) {
        ensure_input_length_match(tokens.len(), amounts.len());
        let mut count = 0;
        while count < tokens.len() {
            let token = tokens.get(count).unwrap();
            let amount = amounts.get(count).unwrap();
            transfer_to_output(amount, token, recipient);
            count = count + 1;
        }
    }

    #[storage(write)]
    fn set_swap_fee_percentage(new_swap_fee_percentage: u64) {
        require(
            new_swap_fee_percentage <= _MAX_PROTOCOL_SWAP_FEE_PERCENTAGE, 
            Error::swap_fee_percentage_too_high
        );
        storage.swap_fee_percentage = new_swap_fee_percentage;
        log(
            SwapFeePercentageChanged {
                new_swap_fee_percentage
            }
        );
    }

    #[storage(write)]
    fn set_flash_loan_fee_percentage(new_flash_loan_fee_percentage: u64) {
        require(
            new_flash_loan_fee_percentage <= _MAX_PROTOCOL_FLASH_LOAN_FEE_PERCENTAGE,
            Error::flash_loan_fee_percentage_too_high
        );
        storage.flash_loan_fee_percentage = new_flash_loan_fee_percentage;
        log(
            FlashLoanFeePercentageChanged {
                new_flash_loan_fee_percentage
            }
        );
    }

    #[storage(read)]
    fn get_swap_fee_percentage() -> u64 {
        return storage.swap_fee_percentage;
    }

    #[storage(read)]
    fn get_flash_loan_fee_percentage() -> u64 {
        return storage.flash_loan_fee_percentage;
    }

    fn get_collected_fee_amounts(tokens: Vec<ContractId>) -> Vec<u64>{
        let mut feeAmounts = ~Vec::new();
        let mut count = 0;
       
        while count < tokens.len() {
            feeAmounts.push(balance_of(contract_id(), tokens.get(count).unwrap()));
            count = count + 1;
        }
        return feeAmounts;
    }

    // this below function originally belong to fee contract
    // Returns the protocol fee amount to charge for a flash loan of `amount`.
    #[storage(read)]
    fn calculate_flash_loan_fee_amount(amount: u64) -> u64 {
        // Fixed point multiplication introduces error: we round up, which means in certain scenarios the charged
        // percentage can be slightly higher than intended.
        let percentage = storage.flash_loan_fee_percentage;
        return mul_up(amount, percentage);
    }

    fn pay_fee_amount(token: ContractId, amount: u64) {
        if (amount > 0) {
            let address: b256 = contract_id().into();
            transfer_to_output(amount, token, ~Address::from(address));
        }
    }
}
