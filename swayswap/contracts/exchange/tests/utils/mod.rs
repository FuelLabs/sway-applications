use fuels::{prelude::*, tx::AssetId};

abigen!(Exchange, "out/debug/exchange-abi.json");
abigen!(MyToken, "../token/out/debug/token-abi.json");

pub async fn deposit_and_add_liquidity(
    exchange_instance: &Exchange,
    native_amount: u64,
    token_amount_deposit: u64,
    token_asset_id: AssetId,
) -> u64 {
    // Deposit some Native Asset
    let _t = exchange_instance
        .deposit()
        .call_params(CallParameters::new(Some(native_amount), None, None))
        .call()
        .await
        .unwrap();

    // Deposit some Token Asset
    let _t = exchange_instance
        .deposit()
        .call_params(CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        ))
        .call()
        .await
        .unwrap();

    // Add liquidity for the second time. Keeping the proportion 1:2
    // It should return the same amount of LP as the amount of ETH deposited
    let result = exchange_instance
        .add_liquidity(1000, 1)
        .call_params(CallParameters::new(
            Some(0),
            Some(token_asset_id.clone()),
            Some(100_000_000),
        ))
        .append_variable_outputs(2)
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .call()
        .await
        .unwrap();

    result.value
}
