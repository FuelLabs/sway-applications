use crate::utils::{expected_liquidity, setup};
use fuels::prelude::*;
use test_utils::{
    abi::{AtomicAddLiquidityScript, SCRIPT_GAS_LIMIT},
    paths::ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH,
};

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

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
async fn when_one_deposit_is_zero() {
    let (wallet, exchange, liquidity_parameters, transaction_parameters) =
        setup((1000, 1000), 1000).await;

    let script_instance =
        AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

    let receipts = script_instance
        .main(
            exchange.id,
            (
                ContractId::new(*exchange.pair.0),
                ContractId::new(*exchange.pair.1),
            ),
            (0, liquidity_parameters.amounts.1),
            1, // if desired liquidity is zero, script will revert with "DesiredLiquidityZero" error
            liquidity_parameters.deadline,
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await;

    dbg!(&receipts);

    receipts.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Revert(18446744073709486080)")]
async fn when_both_deposits_are_zero() {
    let (wallet, exchange, liquidity_parameters, transaction_parameters) =
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
            (0, 0),
            1, // if desired liquidity is zero, script will revert with "DesiredLiquidityZero" error
            liquidity_parameters.deadline,
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await
        .unwrap();
}
