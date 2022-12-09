use fuels::prelude::*;
use test_utils::{
    abi::{exchange::preview_swap_exact_input, SwapExactInputScript},
    data_structures::{AMMContract, TransactionParameters, WalletAssetConfiguration},
    paths::SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_initialize_amm, setup_wallet_and_provider},
        scripts::{setup_exchange_contracts, transaction_inputs_outputs},
    },
};

async fn expected_swap_amounts(
    amm: &AMMContract,
    input_amount: u64,
    route: &Vec<AssetId>,
) -> Vec<u64> {
    assert!(route.len() >= 2);
    let (mut i, mut amounts) = (0, vec![input_amount]);

    while i < route.len() - 1 {
        let pair = (*route.get(i).unwrap(), *route.get(i + 1).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        let amount = preview_swap_exact_input(&exchange, amounts[i], pair.0)
            .await
            .value
            .amount;
        amounts.push(amount);
        i += 1;
    }
    amounts
}

async fn setup() -> (
    WalletUnlocked,
    AMMContract,
    Vec<AssetId>,
    TransactionParameters,
) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let mut amm = deploy_and_initialize_amm(&wallet).await;

    setup_exchange_contracts(&wallet, &mut amm, &asset_ids).await;

    let mut contracts = vec![amm.id];
    contracts.extend(amm.pools.values().into_iter().map(|exchange| exchange.id));

    let transaction_parameters =
        transaction_inputs_outputs(&wallet, &provider, &contracts, &asset_ids, None).await;

    (wallet, amm, asset_ids, transaction_parameters)
}

#[tokio::test]
async fn can_swap_exact_input_along_route() {
    let (wallet, amm, asset_ids, transaction_parameters) = setup().await;

    let route = asset_ids;
    let script_instance =
        SwapExactInputScript::new(wallet.clone(), SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
    let input_amount: u64 = 60;

    let amounts = expected_swap_amounts(&amm, input_amount, &route).await;
    let expected_result = amounts[route.len() - 1];

    let result = script_instance
        .main(
            amm.id,
            route
                .clone()
                .into_iter()
                .map(|asset_id| ContractId::new(*asset_id))
                .collect(),
            amounts,
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(100_000_000), None))
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_result, result);
}
