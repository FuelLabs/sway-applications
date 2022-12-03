script;

use libraries::{AMM, Exchange};
use std::{block::height, constants::ZERO_B256};

pub enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

fn main(
    amm_contract_address: ContractId,
    assets: Vec<ContractId>,
    input_amount: u64,
) -> u64 {
    let amm_contract = abi(AMM, amm_contract_address.into());

    let (mut i, mut latest_output_amount) = (0, input_amount);

    // swap subsequent asset pairs along route by specifying the input asset and the input amount for each swap
    while i < assets.len - 1 {
        let asset_pair = (assets.get(i).unwrap(), assets.get(i + 1).unwrap());

        // get the exchange contract id of asset pair
        let exchange_contract_id = amm_contract.pool {
            gas: 100_000,
            coins: 0,
            asset_id: ZERO_B256,
        }(asset_pair);

        require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));

        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());

        // these parameters are placeholders and will change as the script evolves
        let minimum_output: Option<u64> = Option::None;
        let deadline = height() + 5;

        // initially, forward the input amount passed to the script
        // for the following swaps, forward the previous output amount
        latest_output_amount = exchange_contract.swap_exact_input {
            gas: 10_000_000,
            coins: latest_output_amount,
            asset_id: asset_pair.0.into(),
        }(minimum_output, deadline);

        i += 1;
    }

    latest_output_amount
}
