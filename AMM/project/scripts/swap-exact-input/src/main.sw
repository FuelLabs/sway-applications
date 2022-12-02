script;

use core::num::*;
use libraries::{AMM, Exchange};
use std::{constants::ZERO_B256, logging::log};

pub enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

// It is assumed that the path validity is checked while running this script such that consecutive assets have a valid exchange contract.
fn main(
    amm_contract_address: ContractId,
    assets: Vec<ContractId>,
    input_amount: u64,
) -> u64 {
    let amm_contract = abi(AMM, amm_contract_address.into());
    let (mut i, mut output_amount, path_length) = (0, Option::None::<u64>(), assets.len);
    while i < path_length - 1 {
        let asset_pair = (assets.get(i).unwrap(), assets.get(i + 1).unwrap());
        let exchange_contract_id = amm_contract.pool {
            gas: 100_000,
            coins: 0,
            asset_id: ZERO_B256,
        }(asset_pair);
        require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));
        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());
        let minimum_output: Option<u64> = Option::None;
        output_amount = Option::Some(exchange_contract.swap_exact_input {
            gas: 10_000_000,
            coins: input_amount,
            asset_id: asset_pair.0.into(),
        }(minimum_output, 1000));
        i += 1;
    }
    output_amount.unwrap()
}
