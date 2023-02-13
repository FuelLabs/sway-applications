script;

use libraries::{AMM, data_structures::Asset, Exchange};

enum InputError {
    RouteTooShort: (),
}

enum SwapError {
    ExcessiveSlippage: u64,
    PairExchangeNotRegistered: (ContractId, ContractId),
}

fn main(
    assets: Vec<ContractId>,
    output_amount: u64,
    maximum_input_amount: u64,
    deadline: u64,
) -> u64 {
    require(assets.len() >= 2, InputError::RouteTooShort);

    let amm_contract = abi(AMM, AMM_ID);

    let mut latest_sold = output_amount;

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

        let preview = exchange_contract.preview_swap_exact_output(Asset::new(asset_pair.1, latest_sold));
        let sell_amount = preview.other_asset.amount;

        // swap by specifying the exact amount to buy
        latest_sold = exchange_contract.swap_exact_output {
            gas: 10_000_000,
            coins: sell_amount, // forward coins of asset to sell
            asset_id: asset_pair.0.into(), // identifier of asset to sell
        }(latest_sold, deadline);

        bought_asset_index -= 1;
    }

    require(latest_sold <= maximum_input_amount, SwapError::ExcessiveSlippage(latest_sold));

    latest_sold
}
