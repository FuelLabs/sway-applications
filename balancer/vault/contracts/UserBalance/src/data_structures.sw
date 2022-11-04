library data_structures;

use std::{
    address::Address,
    contract_id::ContractId,
};

pub enum UserBalanceOpKind { 
    DEPOSIT_INTERNAL: (),
    TRANSFER_EXTERNAL: (), 
    TRANSFER_INTERNAL: (),
    WITHDRAW_INTERNAL: (),
}

pub struct UserBalanceOp {
    amount: u64,
    asset: ContractId,
    kind: UserBalanceOpKind,
    recipient: Address,
    sender: Address,
}