contract;


use std::{
    contract_id::ContractId,
    address::Address,
    option::Option,
    vec::Vec,
    context::balance_of,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::*,
    token::transfer_to_output,
    revert::{revert, require},
};

use FixedPoint::mul_up;
// use ProtocolFeesCollector::ProtocolFeesCollector;

abi Fees {
    fn _get_protocol_swap_fee_percentage() -> u64;
    fn _calculate_flash_loan_fee_amount(amount: u64) -> u64;
    fn _pay_fee_amount(token: ContractId, amount: u64);
}

storage {
    protocol_fees_collector_cotract_id: b256 = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0,
}

// To reduce the bytecode size of the Vault, most of the protocol fee logic is not here, but in the
// ProtocolFeesCollector contract.
impl Fees for Contract {
    // Returns the protocol swap fee percentage.
    fn _get_protocol_swap_fee_percentage() -> u64 {
        // let x = abi(protocolFeesCollector, protocol_fees_collector_cotract_id);
        // return x.getSwapFeePercentage();
        0
    }

    // Returns the protocol fee amount to charge for a flash loan of `amount`.
    fn _calculate_flash_loan_fee_amount(amount: u64) -> u64 {
        // Fixed point multiplication introduces error: we round up, which means in certain scenarios the charged
        // percentage can be slightly higher than intended.
        let percentage = getFlashLoanFeePercentage();
        return mul_up(amount, percentage);
    }

    fn _pay_fee_amount(token: ContractId, amount: u64) {
        if (amount > 0) {
            transfer_to_output(amount, ~ContractId::from(storage.protocolFeesCollector_cotract_id), token);
        }
    }
}
