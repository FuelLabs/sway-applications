use crate::utils::{expected_liquidity, setup};
use test_utils::{
    data_structures::LiquidityParameters as TestLiquidityParameters,
    interface::{
        abigen_bindings::shared_types::{Asset, AssetPair},
        LiquidityParameters,
    },
    setup::common::deposit_and_add_liquidity,
};

#[tokio::test]
async fn adds_liquidity_with_equal_deposit_amounts() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add initial liquidity with amounts 1000:1000
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_liquidity_to_make_a_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 2000), 1000).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add initial liquidity with amounts 1000:2000
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_liquidity_to_make_b_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((2000, 1000), 1000).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add initial liquidity with amounts 2000:1000
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_without_extra_deposit_when_a_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 4000), 2000).await;

    // add initial liquidity with amounts 1000:4000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((1000, 4000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add further liquidity with amounts 1000:4000 i.e. no extra deposit
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_with_extra_a_deposit_when_a_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((200, 400), 200).await;

    // add initial liquidity with amounts 1000:4000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((1000, 4000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add further liquidity with amounts 200:400 i.e. depositing extra 100 of asset A
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_with_extra_b_deposit_when_a_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((100, 500), 200).await;

    // add initial liquidity with amounts 1000:4000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((1000, 4000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, true).await;

    // add further liquidity with amounts 100:500 i.e. depositing extra 100 of asset B
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_without_extra_deposit_when_b_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((400, 100), 200).await;

    // add initial liquidity with amounts 4000:1000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((4000, 1000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add further liquidity with amounts 400:100 i.e. no extra deposit
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_with_extra_a_deposit_when_b_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((500, 100), 200).await;

    // add initial liquidity with amounts 4000:1000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((4000, 1000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    // add further liquidity with amounts 500:100 i.e. depositing extra 100 of asset A
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}

#[tokio::test]
async fn adds_further_liquidity_with_extra_b_deposit_when_b_is_more_valuable() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((400, 200), 200).await;

    // add initial liquidity with amounts 4000:1000
    let initial_liquidity_parameters = TestLiquidityParameters::new(
        Some((4000, 1000)),
        Some(liquidity_parameters.deadline),
        Some(2000),
    );
    deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange, false).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, true).await;

    // add further liquidity with amounts 400:200 i.e. depositing extra 100 of asset B
    let liquidity = script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: expected_liquidity,
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_contracts(&[&exchange.instance])
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_liquidity, liquidity);
}
