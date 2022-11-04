contract;

dep interface;

use interface::FlashLoans;

use std::{
    vec::Vec,
    contract_id::ContractId,
    constants::ZERO_B256,
    context::call_frames::contract_id,
    token::transfer_to_output,
    context::balance_of,
    option::Option,
    revert::require,
    address::Address,
};

use InputHelpers::ensure_input_length_match;
// use ProtocolFeesCollector::ProtocolFeesCollector;


storage {  
    valut_contract_id: b256 = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0,
    protocol_fees_collector_contract_id: b256 = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0,
}



// Handles Flash Loans through the Vault. Calls the `receiveFlashLoan` hook on the flash loan recipient
// contract, which implements the `IFlashLoanRecipient` interface.
impl FlashLoans for Contract {
    #[storage(read, write)]
    fn flash_loan(
        // IFlashLoanRecipient recipient,
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        userData: Vec<b256>,
    ) {
        ensure_input_length_match(tokens.len(), amounts.len());

        let mut feeAmounts = ~Vec::new();
        let mut preLoanBalances = ~Vec::new();

        // Used to ensure `tokens` is sorted in ascending order, which ensures token uniqueness.
        let mut previousToken: ContractId = ~ContractId::from(ZERO_B256);
        let firstToken = previousToken;
        let mut count = 0;
        while count < tokens.len() {
            // let token = tokens.get(count).unwrap();
            // let amount = amounts.get(count).unwrap();

            // if token == firstToken {
            //     let token: b256 = token.into();
            //     let previousToken: b256 = previousToken.into();
            //     require(token > previousToken, "ZERO_TOKEN");
            // }
            // else {
            //     let token: b256 = token.into();
            //     let previousToken: b256 = previousToken.into();
            //     require(token > previousToken, "UNSORTED_TOKENS");                
            // }
            // previousToken = token;

            // preLoanBalances.push(balance_of(token, contract_id()));
            // let x = abi(ProtocolFeesCollector, storage.protocol_fees_collector_contract_id);
            // feeAmounts.push(x.calculate_flash_loan_fee_amount(amount));

            // require(preLoanBalances.get(count).unwrap() >= amount, "INSUFFICIENT_FLASH_LOAN_BALANCE");
            // todo need to discuss this
            // transfer_to_output(amount, token, recipient);
            count = count + 1;
        }

        // recipient.receiveFlashLoan(tokens, amounts, feeAmounts, userData);
        // implimnetation of above function
        // require(msg.sender == address(_vault), Errors.CALLER_NOT_VAULT);
        transfer_to_output(amounts.get(0).unwrap(), tokens.get(0).unwrap(), ~Address::from(storage.valut_contract_id));
        count = 0;
        // while count < tokens.len() {
        //     let token = tokens.get(count).unwrap();
        //     let preLoanBalance = preLoanBalances.get(count).unwrap();

        //     // Checking for loan repayment first (without accounting for fees) makes for simpler debugging, and results
        //     // in more accurate revert reasons if the flash loan protocol fee percentage is zero.
        //     let postLoanBalance = balance_of(contract_id(), token);
        //     require(postLoanBalance >= preLoanBalance, "INVALID_POST_LOAN_BALANCE");

        //     // No need for checked arithmetic since we know the loan was fully repaid.
        //     let receivedFeeAmount = postLoanBalance - preLoanBalance;
        //     require(receivedFeeAmount >= feeAmounts.get(count).unwrap(), "INSUFFICIENT_FLASH_LOAN_FEE_AMOUNT");

        //     // let x = abi(ProtocolFeesCollector, protocol_fees_collector_contract_id);
        //     // x.pay_fee_amount(token, receivedFeeAmount);
        //     // emit FlashLoan(recipient, token, amounts[i], receivedFeeAmount);
        //     count = count + 1;
        // }
    }
}
