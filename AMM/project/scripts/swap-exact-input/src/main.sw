script;

use libraries::{AMM, Exchange};

enum InputError {
    RouteTooShort: (),
}

enum SwapError {
    ExcessiveSlippage: u64,
    PairExchangeNotRegistered: (ContractId, ContractId),
}

configurable {
    AMM_ID: b256 = 0x48b3ac8138e5628d29e376e793d23ef5c68de44bee18f128b37641979df754e1,
}

fn main(
    assets: Vec<ContractId>,
    input_amount: u64,
    minimum_output_amount: Option<u64>,
    deadline: u64,
) -> u64 {
    require(assets.len() >= 2, InputError::RouteTooShort);

    let amm_contract = abi(AMM, AMM_ID);

    let mut latest_bought = input_amount;

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

        // swap by specifying the exact amount to sell
        latest_bought = exchange_contract.swap_exact_input {
            gas: 10_000_000,
            coins: latest_bought, // forwarding coins of asset to sell
            asset_id: asset_pair.0.into(), // identifier of asset to sell
        }(Option::None, deadline);

        sold_asset_index += 1;
    }

    if minimum_output_amount.is_some() {
        require(latest_bought >= minimum_output_amount.unwrap(), SwapError::ExcessiveSlippage(latest_bought));
    }

    latest_bought
}
