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

    let mut latest_bought = amounts.get(1).unwrap();
    let deadline = height() + 5;

    // start swapping by selling the first asset in the route
    let mut sold_asset_index = 0;

    // swap subsequent asset pairs along route
    while sold_asset_index < assets.len - 1 {
        let asset_pair = (
            assets.get(sold_asset_index).unwrap(),
            assets.get(sold_asset_index + 1).unwrap(),
        );

        // get the exchange contract id of asset pair
        let exchange_contract_id = amm_contract.pool { gas: 100_000 }(asset_pair);

        require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));

        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());

        let minimum_buy_amount = Option::Some(latest_bought);

        // swap by specifying the exact amount to sell
        latest_bought = exchange_contract.swap_exact_input {
            gas: 10_000_000,
            coins: amounts.get(sold_asset_index).unwrap(), // forwarding coins of asset to sell
            asset_id: asset_pair.0.into(), // identifier of asset to sell
        }(minimum_buy_amount, deadline);

        sold_asset_index += 1;
    }

    latest_bought
}
