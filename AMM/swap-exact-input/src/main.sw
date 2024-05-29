script;

use libraries::{AMM, Exchange};

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

/// Swaps assets along a route by specifying exact input for each swap.
///
/// # Arguments
///
/// * `assets`: [Vec<AssetId>] - The assets along the swap route.
/// * `input_amount`: [u64] - The desired amount of the input asset.
/// * `minimum_output_amount`: [u64] - The maximum amount of the output asset.
/// * `deadline`: [u64] - The limit on block height for operation.
///
/// # Returns
///
/// * The amount of the output asset.
///
/// # Reverts
///
/// * When `assets.len()` is less than two.
/// * When the exchange contract has not been registered in the AMM.
/// * When the amount of the brought asset is less than `minimum_output_amount`.
fn main(
    assets: Vec<AssetId>,
    input_amount: u64,
    minimum_output_amount: Option<u64>,
    deadline: u64,
) -> u64 {
    require(assets.len() >= 2, InputError::RouteTooShort);

    let amm_contract = abi(AMM, AMM_ID);

    let mut latest_bought = input_amount;

    // start swapping by selling the first asset in the route.
    let mut sold_asset_index = 0;

    // swap subsequent asset pairs along route.
    while sold_asset_index < assets.len() - 1 {
        let asset_pair = (
            assets.get(sold_asset_index).unwrap(),
            assets.get(sold_asset_index + 1).unwrap(),
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

        // swap by specifying the exact amount to sell.
        latest_bought = exchange_contract.swap_exact_input {
            gas: 10_000_000,
            coins: latest_bought, // forwarding coins of asset to sell.
            asset_id: asset_pair.0.into(), // identifier of asset to sell.
        }(Option::None, deadline);

        sold_asset_index += 1;
    }

    if minimum_output_amount.is_some() {
        require(
            latest_bought >= minimum_output_amount
                .unwrap(),
            SwapError::ExcessiveSlippage(latest_bought),
        );
    }

    latest_bought
}
