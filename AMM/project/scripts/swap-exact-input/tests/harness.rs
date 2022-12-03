use crate::utils::{
    amounts::MAXIMUM_INPUT_AMOUNT,
    expected_swap_output,
    paths::SCRIPT_BINARY_PATH,
    setup::setup,
    transaction::{
        transaction_input_coin, transaction_input_contract, transaction_output_contract,
        transaction_output_variable,
    },
    SwapScript,
};
use fuel_gql_client::prelude::*;
use fuels::prelude::*;

mod utils;

#[tokio::test]
async fn can_swap_exact_input_along_route() {
    let (wallet, provider, amm, asset_ids) = setup().await;
    let script_instance = SwapScript::new(wallet.clone(), SCRIPT_BINARY_PATH);

    let input_amount: u64 = 60;

    // TODO (@supiket): investigate why adding the last asset's coins as transaction inputs does not fail the test
    let route = vec![
        asset_ids[0],
        asset_ids[1],
        asset_ids[2],
        asset_ids[3],
        asset_ids[4],
    ];

    dbg!(route.clone());

    let mut input_contracts: Vec<Input> = vec![transaction_input_contract(amm.id)];
    let mut output_contracts: Vec<Output> = vec![transaction_output_contract(0)];

    amm.pools
        .values()
        .into_iter()
        .enumerate()
        .for_each(|(index, pool)| {
            input_contracts.push(transaction_input_contract(pool.id));
            output_contracts.push(transaction_output_contract(index as u8 + 1));
        });

    let mut input_coins: Vec<Input> = vec![];
    let mut output_variables: Vec<Output> = vec![];

    let mut i = 0;
    while i < route.len() {
        input_coins.extend(
            transaction_input_coin(
                &provider,
                wallet.address(),
                *route.get(i).unwrap(),
                MAXIMUM_INPUT_AMOUNT,
            )
            .await,
        );
        output_variables.push(transaction_output_variable());
        i += 1;
    }

    let expected_result = expected_swap_output(&amm, input_amount, route.clone()).await;

    let result = script_instance
        .main(
            amm.id,
            route
                .clone()
                .into_iter()
                .map(|asset_id| ContractId::new(*asset_id))
                .collect(),
            input_amount,
        )
        .with_inputs([input_contracts, input_coins].concat())
        .with_outputs([output_contracts, output_variables].concat())
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_result, result);
}
