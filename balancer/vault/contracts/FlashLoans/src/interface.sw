library interface;

use std::{
    contract_id::ContractId,
    vec::Vec,
};

abi FlashLoans {
    #[storage(read, write)]
    fn flash_loan(
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        userData: Vec<b256>,
    );
}