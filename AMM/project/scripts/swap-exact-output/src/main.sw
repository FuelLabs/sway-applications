script;

use libraries::{AMM, Exchange};
use std::block::height;

enum InputError {
    RouteTooShort: (),
    SwapAmountsNotExact: (),
}

enum SwapError {
    PairExchangeNotRegistered: (ContractId, ContractId),
}

fn main(
    amm_contract_address: ContractId,
    assets: Vec<ContractId>,
    amounts: Vec<u64>,
) -> u64 {
    require(assets.len() >= 2, InputError::RouteTooShort);
    require(amounts.len() == assets.len(), InputError::SwapAmountsNotExact);

    let amm_contract = abi(AMM, amm_contract_address.into());

    let mut latest_sold = Option::None;
    let deadline = height() + 5;

    // start swapping by buying the last asset in the route
    let mut bought_asset_index = assets.len() - 1;

    // swap subsequent asset pairs along route
    while bought_asset_index > 0 {
        let asset_pair = (
            assets.get(bought_asset_index - 1).unwrap(),
            assets.get(bought_asset_index).unwrap(),
        );

        // get the exchange contract id of asset pair
        let exchange_contract_id = amm_contract.pool { gas: 100_000 }(asset_pair);

        require(exchange_contract_id.is_some(), SwapError::PairExchangeNotRegistered(asset_pair));

        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());

        latest_sold = Option::Some(amounts.get(bought_asset_index - 1).unwrap());
        let maximum_sell_amount = amounts.get(bought_asset_index).unwrap();

        // swap by specifying the exact amount to buy
        let _sold = exchange_contract.swap_exact_output {
            gas: 10_000_000,
            coins: latest_sold.unwrap(), // forward coins of asset to sell
            asset_id: asset_pair.0.into(), // identifier of asset to sell
        }(maximum_sell_amount, deadline);

        bought_asset_index -= 1;
    }

    latest_sold.unwrap()
}
