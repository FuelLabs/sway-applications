use crate::utils::{expected_liquidity, setup};
use fuels::prelude::*;
use test_utils::interface::{
    atomic_add_liquidity_script_mod::{Asset, AssetPair},
    LiquidityParameters, SCRIPT_GAS_LIMIT,
};

#[tokio::test]
#[should_panic(expected = "DesiredLiquidityZero")]
async fn when_desired_liquidity_zero() {
    let (script_instance, exchange, liquidity_parameters, _transaction_parameters) =
        setup((1000, 1000), 1000).await;

    script_instance
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
                liquidity: 0, // desired liquidity is 0
                deadline: liquidity_parameters.deadline,
            },
        )
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
// the contract call in the script fails with "DesiredAmountTooHigh" but that message is not propagated
async fn when_desired_liquidity_too_high() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters, false).await;

    script_instance
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
                liquidity: expected_liquidity + 1, //desired liquidity is too high
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
async fn when_one_deposit_is_zero() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: 0, // deposit amount is 0
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: 1, // if desired liquidity is zero, script will revert with "DesiredLiquidityZero" error
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
async fn when_both_deposits_are_zero() {
    let (script_instance, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    script_instance
        .main(
            exchange.id,
            LiquidityParameters {
                deposits: AssetPair {
                    a: Asset {
                        id: ContractId::new(*exchange.pair.0),
                        amount: 0, // deposit amount is 0
                    },
                    b: Asset {
                        id: ContractId::new(*exchange.pair.1),
                        amount: 0, // deposit amount is 0
                    },
                },
                liquidity: 1, // if desired liquidity is zero, script will revert with "DesiredLiquidityZero" error
                deadline: liquidity_parameters.deadline,
            },
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await
        .unwrap();
}
