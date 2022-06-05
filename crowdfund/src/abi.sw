library abi;

dep data_structures;

use std::{chain::auth::Sender, contract_id::ContractId};
use data_structures::Info;

abi Crowdfund {
    fn constructor(author: Sender, asset: ContractId, target_amount: u64, deadline: u64) -> bool;
    fn pledge() -> bool;
    fn unpledge(amount: u64) -> bool;
    fn claim() -> bool;
    fn status() -> Info;
}
