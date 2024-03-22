script;

use libraries::{data_structures::LiquidityParameters, Exchange};

/// Determines the type of input error.
enum InputError {
    /// The desired liquidity is zero.
    DesiredLiquidityZero: (),
}

/// Deposits pool assets and adds liquidity to an AMM exchange contract.
///
/// # Arguments
///
/// * `exchange_contract_id`: [ContractId] - The contract id of the exchange.
/// * `liquidity_parameters`: [LiquidityParameters] - Exchange liquidity for a specific asset pair.
///
/// # Returns
///
/// * The amount of liquidity assets minted.
///
/// # Reverts
///
/// * When `liquidity_parameters.liquidity` is not greater than zero.
fn main(
    exchange_contract_id: ContractId,
    liquidity_parameters: LiquidityParameters,
) -> u64 {
    require(
        liquidity_parameters
            .liquidity > 0,
        InputError::DesiredLiquidityZero,
    );

    let exchange_contract = abi(Exchange, exchange_contract_id.into());

    // deposit first asset
    exchange_contract
        .deposit {
            gas: 70_000,
            coins: liquidity_parameters.deposits.a.amount,
            asset_id: liquidity_parameters.deposits.a.id.into(),
        }();

    // deposit second asset
    exchange_contract
        .deposit {
            gas: 70_000,
            coins: liquidity_parameters.deposits.b.amount,
            asset_id: liquidity_parameters.deposits.b.id.into(),
        }();

    // add liquidity
    exchange_contract.add_liquidity {
        gas: 15_000_000,
    }(
        liquidity_parameters
            .liquidity,
        liquidity_parameters
            .deadline,
    )
}
