use crate::utils::{expected_liquidity, setup};
use fuels::prelude::*;
use test_utils::{
    data_structures::LiquidityParameters as TestLiquidityParameters,
    interface::{
        atomic_add_liquidity_script_mod::{Asset, AssetPair},
        AtomicAddLiquidityScript, LiquidityParameters, SCRIPT_GAS_LIMIT,
    },
    paths::ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH,
    setup::common::deposit_and_add_liquidity,
};

#[tokio::test]
async fn adds_liquidity_with_equal_deposit_amounts() {
    let (wallet, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    let script_instance =
        AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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
async fn adds_liquidity_to_make_a_more_valuable() {
    let (wallet, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 2000), 1000).await;

    let script_instance =
        AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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
async fn adds_liquidity_to_make_b_more_valuable() {
    let (wallet, exchange, liquidity_parameters, transaction_parameters) =
        setup((2000, 1000), 1000).await;

    let script_instance =
        AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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

    let initial_liquidity_parameters = TestLiquidityParameters::new(
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
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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

    let initial_liquidity_parameters = TestLiquidityParameters::new(
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
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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

    let initial_liquidity_parameters = TestLiquidityParameters::new(
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
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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

    let initial_liquidity_parameters = TestLiquidityParameters::new(
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
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
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
