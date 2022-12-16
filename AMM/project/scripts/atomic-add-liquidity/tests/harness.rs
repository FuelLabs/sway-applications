use fuels::prelude::*;
use test_utils::{
    abi::{
        exchange::{deposit, preview_add_liquidity, withdraw},
        AtomicAddLiquidityScript, SCRIPT_GAS_LIMIT,
    },
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        TransactionParameters, WalletAssetConfiguration,
    },
    paths::ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH,
    setup::{
        common::{
            deploy_and_construct_exchange, deposit_and_add_liquidity, setup_wallet_and_provider,
        },
        scripts::transaction_inputs_outputs,
    },
};

async fn expected_liquidity(
    exchange: &ExchangeContract,
    liquidity_parameters: &LiquidityParameters,
) -> u64 {
    deposit(
        &exchange.instance,
        liquidity_parameters.amounts.0,
        exchange.pair.0,
    )
    .await;

    let preview_add_liquidity_info = preview_add_liquidity(
        &exchange.instance,
        liquidity_parameters.amounts.1,
        exchange.pair.1,
        true,
    )
    .await;

    withdraw(
        &exchange.instance,
        liquidity_parameters.amounts.0,
        exchange.pair.0,
    )
    .await;

    preview_add_liquidity_info.liquidity_asset_amount_to_receive
}

async fn setup(
    deposit_amounts: (u64, u64),
    liquidity: u64,
) -> (
    WalletUnlocked,
    ExchangeContract,
    LiquidityParameters,
    TransactionParameters,
) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let exchange = deploy_and_construct_exchange(
        &wallet,
        &ExchangeContractConfiguration::new(Some((asset_ids[0], asset_ids[1])), None, None, None),
    )
    .await;

    let liquidity_parameters = LiquidityParameters::new(
        Some(deposit_amounts),
        Some(provider.latest_block_height().await.unwrap() + 10),
        Some(liquidity),
    );

    let transaction_parameters = transaction_inputs_outputs(
        &wallet,
        &provider,
        &vec![exchange.id],
        &vec![*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap()],
        Some(&vec![
            liquidity_parameters.amounts.0,
            liquidity_parameters.amounts.1,
        ]),
    )
    .await;

    (
        wallet,
        exchange,
        liquidity_parameters,
        transaction_parameters,
    )
}

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_initial_liquidity_with_equal_deposit_amounts() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((1000, 1000), 1000).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_initial_liquidity_to_make_a_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((1000, 2000), 1000).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_initial_liquidity_to_make_b_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((2000, 1000), 1000).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_without_extra_deposit_when_a_is_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((1000, 4000), 2000).await;

        let initial_liquidity_parameters = LiquidityParameters::new(
            Some((1000, 4000)),
            Some(liquidity_parameters.deadline),
            Some(2000),
        );
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_with_extra_a_deposit_when_a_is_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((200, 200), 100).await;

        let initial_liquidity_parameters = LiquidityParameters::new(
            Some((1000, 4000)),
            Some(liquidity_parameters.deadline),
            Some(2000),
        );
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_without_extra_deposit_when_b_is_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((400, 50), 100).await;

        let initial_liquidity_parameters = LiquidityParameters::new(
            Some((4000, 1000)),
            Some(liquidity_parameters.deadline),
            Some(2000),
        );
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_with_extra_a_deposit_when_b_is_more_valuable() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((200, 50), 100).await;

        let initial_liquidity_parameters = LiquidityParameters::new(
            Some((4000, 1000)),
            Some(liquidity_parameters.deadline),
            Some(2000),
        );
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "DesiredLiquidityZero")]
    async fn when_desired_liquidity_zero() {
        let (wallet, exchange, liquidity_parameters, _transaction_parameters) =
            setup((1000, 1000), 1000).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                0, // desired liquidity is 0
                liquidity_parameters.deadline,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    // the contract call in the script fails with "DesiredAmountTooHigh" but that message is not propagated
    async fn when_desired_liquidity_too_high() {
        let (wallet, exchange, liquidity_parameters, transaction_parameters) =
            setup((1000, 1000), 1000).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity + 1, //desired liquidity is too high
                liquidity_parameters.deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap();
    }
}
