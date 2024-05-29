script;

use libraries::{AMM, data_structures::Asset, data_structures::PreviewSwapInfo, Exchange};

/// Determines the type of input error.
enum InputError {
    /// The number of assets in the swap is less than 2.
    RouteTooShort: (),
}

/// Determines the type of swap error.
enum SwapError {
    /// The amount bought is less than the minimum output amount.
    ExcessiveSlippage: u64,
    /// The exchange for this asset pair could not be found.
    PairExchangeNotRegistered: (AssetId, AssetId),
}

configurable {
    /// The ContractId of the AMM contract.
    AMM_ID: b256 = 0x8aea4274cd6fcc79094c55fb3c065046b6c759c2169786bc350536660eaba670,
}

/// Swaps assets along a route by specifying exact output for each swap.
///
/// # Arguments
///
/// * `assets`: [Vec<AssetId>] - The assets along the swap route.
/// * `output_amount`: [u64] - The desired amount of the output asset.
/// * `maximum_input_amount`: [u64] - The maximum amount of the input asset.
/// * `deadline`: [u64] - The limit on block height for operation.
///
/// # Returns
///
/// * `u64`: The amount of the input asset that was sold.
///
/// # Reverts
///
/// * When `assets.len()` is less than two.
/// * When the exchange contract has not been registered in the AMM.
/// * When the amount of the sold asset is greater than `maximum_input_amount`.
fn main(
    assets: Vec<AssetId>,
    output_amount: u64,
    maximum_input_amount: u64,
    deadline: u64,
) -> u64 {
    require(assets.len() >= 2, InputError::RouteTooShort);

    let amm_contract = abi(AMM, AMM_ID);

    let mut latest_sold = output_amount;

    // start swapping by buying the last asset in the route.
    let mut bought_asset_index = assets.len() - 1;

    // swap subsequent asset pairs along route.
    while bought_asset_index > 0 {
        let asset_pair = (
            assets.get(bought_asset_index - 1).unwrap(),
            assets.get(bought_asset_index).unwrap(),
        );

        // get the exchange contract id of asset pair.
        let exchange_contract_id = amm_contract.pool {
            gas: 100_000,
        }(asset_pair);

        require(
            exchange_contract_id
                .is_some(),
            SwapError::PairExchangeNotRegistered(asset_pair),
        );

        let exchange_contract = abi(Exchange, exchange_contract_id.unwrap().into());

        let preview = exchange_contract.preview_swap_exact_output(Asset::new(asset_pair.1, latest_sold));
        let sell_amount = preview.other_asset.amount;

        // swap by specifying the exact amount to buy.
        latest_sold = exchange_contract.swap_exact_output {
            gas: 10_000_000,
            coins: sell_amount, // forward coins of asset to sell.
            asset_id: asset_pair.0.into(), // identifier of asset to sell.
        }(latest_sold, deadline);

        bought_asset_index -= 1;
    }

    require(
        latest_sold <= maximum_input_amount,
        SwapError::ExcessiveSlippage(latest_sold),
    );

    latest_sold
}
