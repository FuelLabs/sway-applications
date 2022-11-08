script;

use core::num::*;
use libraries::{AMM, Exchange};
use std::{logging::log, prelude::*};

pub enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

// It is assumed that the path validity is checked while running this script such that consecutive assets have a valid exchange contract.
fn main(
    amm_contract_address: ContractId,
    asset_0_id: ContractId,
    asset_1_id: ContractId,
    asset_2_id: ContractId,
    input_amount: u64,
) -> u64 {
    let amm_contract = abi(AMM, amm_contract_address.into());
    let asset_pair_0 = (asset_0_id, asset_1_id);
    let exchange_contract_id = amm_contract.pool(asset_pair_0);
    require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair_0));
    let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
    let mut output_amount = exchange_contract.swap_with_exact_input {
        gas: 10_000_000,
        coins: input_amount,
        asset_id: asset_0_id.into(),
    }(Option::None(), 1000);
    let asset_pair_1 = (asset_1_id, asset_2_id);
    let exchange_contract_id = amm_contract.pool(asset_pair_1);
    require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair_1));
    let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
    output_amount = exchange_contract.swap_with_exact_input {
        gas: 10_000_000,
        coins: output_amount,
        asset_id: asset_1_id.into(),
    }(Option::None(), 1000);
    output_amount
}
