script;

use core::num::*;
use libraries::{AMM, Exchange};
use std::{
    constants::ZERO_B256,
    contract_id::ContractId,
    logging::log,
    prelude::*,
    revert::require,
    u128::U128,
};

pub enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

fn main(
    amm_contract_address: ContractId,
    asset_0_id: ContractId,
    asset_1_id: ContractId,
    asset_2_id: ContractId,
    input_amount: u64,
) -> () {
    let amm_contract = abi(AMM, amm_contract_address.into());
    let mut swap_amount = input_amount;
    let asset_pair_0 = (asset_0_id, asset_1_id);
    let exchange_contract_id = amm_contract.pool(asset_pair_0);
    require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair_0));
    let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
    swap_amount = exchange_contract.swap_with_exact_input {
        gas: 10_000_000,
        coins: swap_amount,
        asset_id: asset_0_id.into(),
    }(Option::None(), 1000);
    let asset_pair_1 = (asset_1_id, asset_2_id);
    let exchange_contract_id = amm_contract.pool(asset_pair_1);
    require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair_1));
    let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
    swap_amount = exchange_contract.swap_with_exact_input {
        gas: 10_000_000,
        coins: swap_amount,
        asset_id: asset_1_id.into(),
    }(Option::None(), 1000);
}

// will be replaced with this once vector args are supported
// fn main(
//     amm_contract_address: ContractId,
//     path: Vec<ContractId>,
//     input_amount: u64,
// ) -> () {
//     let amm_contract = abi(AMM, amm_contract_address.into()); 
//     let mut swap_amount = input_amount;
//     let mut i = 0;
//     while i <  path.len() - 1 {
//         let asset_pair = (path.get(i).unwrap(), path.get(i+1).unwrap());
//         let exchange_contract_id = amm_contract.pool(asset_pair);
//         require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));
//         let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
//         swap_amount = exchange_contract.swap_with_exact_input {
//             gas: 10_000_000,
//             coins: swap_amount,
//             asset_id: asset_pair.0.into(),
//         }(Option::None(), 1000);
//         i += 1;
//     }
// }
