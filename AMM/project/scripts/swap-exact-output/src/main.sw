script;

use libraries::{AMM, Exchange};
use std::block::height;

enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

fn main(
    amm_contract_address: ContractId,
    assets: Vec<ContractId>,
    amounts: Vec<u64>,
) -> u64 {
    assert(assets.len() >= 2);

    let amm_contract = abi(AMM, amm_contract_address.into());

    let mut i = assets.len() - 1;

    let mut latest_input = Option::None;
    let deadline = height() + 5;

    while i > 0 {
        let asset_pair = (assets.get(i - 1).unwrap(), assets.get(i).unwrap());

        let exchange_contract_id = amm_contract.pool { gas: 100_000 }(asset_pair);

        require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));

        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());

        latest_input = Option::Some(amounts.get(i - 1).unwrap());

        exchange_contract.swap_exact_output {
            gas: 10_000_000,
            coins: latest_input.unwrap(),
            asset_id: asset_pair.0.into(),
        }(amounts.get(i).unwrap(), deadline);

        i -= 1;
    }

    latest_input.unwrap()
}
